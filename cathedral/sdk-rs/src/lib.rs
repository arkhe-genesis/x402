pub mod crypto;
pub mod grpc;

use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

// ============================================================
// TIPOS DE EVENTO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type")]
pub enum SdkEvent {
    DesignProposed {
        design_hash: String,
        parent_hashes: Vec<String>,
        parameters: HashMap<String, f64>,
        rationale: String,
        agent_id: String,
    },
    SimulationCompleted {
        design_hash: String,
        simulator: String,
        metrics: HashMap<String, f64>,
        convergence: bool,
        compute_cost_usd: f64,
    },
    AgentMutation {
        mutation_description: String,
        previous_agent_hash: String,
        substrate_version: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceResponse {
    pub verdict: String, // approved, rejected, requires_human, conditional, timeout
    pub rationale: String,
    pub conditions: Option<Vec<String>>,
}

// ============================================================
// CONFIGURAÇÃO
// ============================================================

#[derive(Debug, Clone)]
pub struct SdkConfig {
    pub crypto: common::crypto_config::CryptoConfig,
    pub private_key_bytes: Option<Vec<u8>>,
    pub fallback_private_key_bytes: Option<Vec<u8>>,
    pub bridge_endpoint: String,
    pub project_id: String,
    pub agent_id: String,
    pub batch_size: usize,
    pub flush_interval_ms: u64,
    pub governance_mode: GovernanceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceMode {
    HumanInTheLoop,
    AutonomousWithCircuitBreaker,
    AutonomousFull,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            bridge_endpoint: "http://localhost:50051".to_string(),
            crypto: common::crypto_config::CryptoConfig::default(),
            private_key_bytes: None,
            fallback_private_key_bytes: None,
            project_id: "default".to_string(),
            agent_id: "default-agent".to_string(),
            batch_size: 50,
            flush_interval_ms: 5000,
            governance_mode: GovernanceMode::AutonomousWithCircuitBreaker,
        }
    }
}

// ============================================================
// CLIENTE PRINCIPAL
// ============================================================

pub struct CathedralSdk {
    signing_key: crypto::SigningKeyWrapper,
    fallback_key: Option<crypto::SigningKeyWrapper>,
    config: SdkConfig,
    event_tx: mpsc::UnboundedSender<SdkEvent>,
    grpc_client: grpc::GrpcClient,
    metrics: SdkMetrics,
}

struct SdkMetrics {
    events_emitted: u64,
    events_batched: u64,
}

impl CathedralSdk {
    pub async fn new(config: SdkConfig) -> Result<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mut grpc_client = grpc::GrpcClient::new(config.bridge_endpoint.clone()).await?;

        let crypto_config = config.crypto.clone();
        let factory = crypto::CryptoFactory::new(crypto_config.clone());

        let signing_key = if let Some(ref bytes) = config.private_key_bytes {
            crypto::SigningKeyWrapper::from_bytes(crypto_config.signature_algorithm, bytes)?
        } else {
            factory.generate_signing_key()?
        };

        let fallback_key = if crypto_config.dual_stack_mode {
            if let Some(fallback_alg) = crypto_config.fallback_signature_algorithm {
                if let Some(ref fallback_bytes) = config.fallback_private_key_bytes {
                    Some(crypto::SigningKeyWrapper::from_bytes(
                        fallback_alg,
                        fallback_bytes,
                    )?)
                } else {
                    factory.generate_fallback_key()?
                }
            } else {
                None
            }
        } else {
            None
        };

        // Background task para flush em batch
        let config_clone = config.clone();
        let mut grpc_client_clone = grpc_client.clone();

        tokio::spawn(async move {
            let mut batch = Vec::with_capacity(config_clone.batch_size);
            let mut last_flush = tokio::time::Instant::now();

            while let Some(event) = rx.recv().await {
                batch.push(event);
                if batch.len() >= config_clone.batch_size {
                    Self::flush_batch(&mut grpc_client_clone, &config_clone, &mut batch).await;
                    last_flush = tokio::time::Instant::now();
                }
                if last_flush.elapsed().as_millis() as u64 >= config_clone.flush_interval_ms {
                    if !batch.is_empty() {
                        Self::flush_batch(&mut grpc_client_clone, &config_clone, &mut batch).await;
                        last_flush = tokio::time::Instant::now();
                    }
                }
            }
            if !batch.is_empty() {
                Self::flush_batch(&mut grpc_client_clone, &config_clone, &mut batch).await;
            }
        });

        Ok(Self {
            config,
            event_tx: tx,
            grpc_client,
            metrics: SdkMetrics {
                events_emitted: 0,
                events_batched: 0,
            },
            signing_key,
            fallback_key,
        })
    }

    async fn flush_batch(
        client: &mut grpc::GrpcClient,
        config: &SdkConfig,
        batch: &mut Vec<SdkEvent>,
    ) {
        if batch.is_empty() {
            return;
        }

        match client
            .ingest(
                config.project_id.clone(),
                config.agent_id.clone(),
                batch.clone(),
            )
            .await
        {
            Ok(resp) if resp.success => {
                debug!("✅ Batch of {} events sent successfully", batch.len());
            }
            Ok(resp) => {
                error!("❌ Bridge responded with error: {}", resp.message);
            }
            Err(e) => {
                error!("❌ Failed to send batch: {}", e);
            }
        }
        batch.clear();
    }

    // ============================================================
    // EMISSÃO DE EVENTOS
    // ============================================================

    pub async fn emit_design_proposed(
        &self,
        design_hash: String,
        parent_hashes: Vec<String>,
        parameters: HashMap<String, f64>,
        rationale: String,
    ) -> Result<()> {
        let event = SdkEvent::DesignProposed {
            design_hash,
            parent_hashes,
            parameters,
            rationale,
            agent_id: self.config.agent_id.clone(),
        };
        self.event_tx
            .send(event)
            .map_err(|e| anyhow!("Failed to send event: {}", e))?;
        Ok(())
    }

    pub async fn emit_simulation_completed(
        &self,
        design_hash: String,
        simulator: String,
        metrics: HashMap<String, f64>,
        convergence: bool,
        compute_cost_usd: f64,
    ) -> Result<()> {
        let event = SdkEvent::SimulationCompleted {
            design_hash,
            simulator,
            metrics,
            convergence,
            compute_cost_usd,
        };
        self.event_tx
            .send(event)
            .map_err(|e| anyhow!("Failed to send event: {}", e))?;
        Ok(())
    }

    pub async fn request_governance(&mut self, event: SdkEvent) -> Result<GovernanceResponse> {
        if self.config.governance_mode == GovernanceMode::AutonomousFull {
            return Ok(GovernanceResponse {
                verdict: "approved".to_string(),
                rationale: "Autonomous full mode".to_string(),
                conditions: None,
            });
        }

        let risk = Self::estimate_risk(&event);
        if self.config.governance_mode == GovernanceMode::AutonomousWithCircuitBreaker && risk < 0.5
        {
            return Ok(GovernanceResponse {
                verdict: "approved".to_string(),
                rationale: "Low risk decision".to_string(),
                conditions: None,
            });
        }

        self.grpc_client
            .request_governance(
                self.config.project_id.clone(),
                self.config.agent_id.clone(),
                event,
            )
            .await
    }

    fn estimate_risk(event: &SdkEvent) -> f64 {
        match event {
            SdkEvent::AgentMutation { .. } => 0.85,
            SdkEvent::DesignProposed { .. } => 0.20,
            SdkEvent::SimulationCompleted { .. } => 0.30,
        }
    }
}
