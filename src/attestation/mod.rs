pub mod manager;
pub use manager::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAttestation {
    pub id: String,
    pub policy_compliance: bool,
}

impl ExecutionAttestation {
    pub fn is_policy_compliant(&self) -> bool {
        self.policy_compliance
    }
    pub fn policy_attestation_id(&self) -> String {
        "dummy-id".to_string()
    }
}

pub trait AttestationProvider {
    fn run_authorized(
        &self,
        workload: &str,
        cost_cap: Option<f64>,
        identity: &crate::identity_attestation::IdentityAttestation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ExecutionAttestation, String>> + Send + '_>>;
}

pub trait AttestationVerifier {}

pub struct CathedralComputeProvider;

impl CathedralComputeProvider {
    pub fn new(
        _signer: String,
        _nervous_system: String,
        _event_store: String,
        _version: &str,
    ) -> Self {
        CathedralComputeProvider
    }
}

impl AttestationProvider for CathedralComputeProvider {
    fn run_authorized(
        &self,
        workload: &str,
        cost_cap: Option<f64>,
        identity: &crate::identity_attestation::IdentityAttestation,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ExecutionAttestation, String>> + Send + '_>> {
        Box::pin(async {
            Ok(ExecutionAttestation {
                id: "dummy".to_string(),
                policy_compliance: true,
            })
        })
    }
}
