//! Carregador de configuração do agente a partir de arquivos YAML/JSON.
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;

// Mock dependencies para a compilação
#[derive(Debug)]
pub struct OrchestratorError(pub String);

impl OrchestratorError {
    pub fn InvalidTask(s: String) -> Self {
        Self(s)
    }
}

pub struct SphincsSigner;
impl SphincsSigner {
    pub fn new() -> Self {
        Self
    }
}

pub struct MultiAgentOrchestrator {
    pub memory_config: MemoryConfig,
    pub trust_config: TrustConfig,
    pub event_bus: Option<Arc<()>>,
    pub signer: Arc<SphincsSigner>,
}

impl MultiAgentOrchestrator {
    pub fn new(event_bus: Option<Arc<()>>, signer: Arc<SphincsSigner>) -> Self {
        Self {
            memory_config: MemoryConfig {
                short_term_capacity: 0,
                long_term_enabled: false,
                vector_db: String::new(),
            },
            trust_config: TrustConfig {
                require_memory_proof: false,
                require_spex: false,
                post_quantum_signature: false,
            },
            event_bus,
            signer,
        }
    }

    pub async fn new_with_config(
        config_path: &str,
        manifest_path: &str,
    ) -> Result<Self, OrchestratorError> {
        let signer = Arc::new(SphincsSigner::new());
        Self::from_config_files(config_path, manifest_path, None, signer).await
    }

    pub async fn from_config_files(
        config_path: &str,
        manifest_path: &str,
        event_bus: Option<Arc<()>>,
        signer: Arc<SphincsSigner>,
    ) -> Result<Self, OrchestratorError> {
        // 1. Carregar config.yaml
        let agent_config = AgentConfigFile::from_yaml(config_path)
            .map_err(|e| OrchestratorError::InvalidTask(format!("Config load error: {}", e)))?;

        // 2. Carregar manifest.json
        let manifest_content = fs::read_to_string(manifest_path)
            .map_err(|e| OrchestratorError::InvalidTask(e.to_string()))?;
        let manifest: serde_json::Value = serde_json::from_str(&manifest_content)
            .map_err(|e| OrchestratorError::InvalidTask(e.to_string()))?;

        // 3. Configurar memória e ferramentas a partir do config
        let memory_cfg = agent_config.agent.memory;
        let trust_cfg = agent_config.agent.trust;

        // 4. Inicializar orquestrador com esses parâmetros
        let mut orchestrator = Self::new(event_bus, signer);
        orchestrator.memory_config = memory_cfg;
        orchestrator.trust_config = trust_cfg;

        // Registar agentes com base nas roles do config (simulação do log)
        println!(
            "Loaded config for agent {} (role: {}) with strategy {}",
            agent_config.agent.id,
            agent_config.agent.role,
            agent_config.agent.planning.strategy
        );
        println!("Loaded manifest for model {:?}", manifest["model_id"]);

        Ok(orchestrator)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentConfigFile {
    pub agent: AgentSection,
    pub governance: GovernanceSection,
    pub telemetry: TelemetrySection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentSection {
    pub id: String,
    pub role: String,
    pub version: String,
    pub system_prompt_path: String,
    pub tools_registry: String,
    pub planning: PlanningConfig,
    pub memory: MemoryConfig,
    pub trust: TrustConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanningConfig {
    pub strategy: String,
    pub max_steps: u32,
    pub consensus_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub short_term_capacity: usize,
    pub long_term_enabled: bool,
    pub vector_db: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TrustConfig {
    pub require_memory_proof: bool,
    pub require_spex: bool,
    pub post_quantum_signature: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GovernanceSection {
    pub constitution: String,
    pub policy_hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelemetrySection {
    pub otel_endpoint: String,
    pub log_level: String,
}

impl AgentConfigFile {
    pub fn from_yaml<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_yaml::from_str(&content).map_err(|e| e.to_string())
    }
}
