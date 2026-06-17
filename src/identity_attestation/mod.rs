use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAttestation {
    pub confidence: f64,
    pub identity_verified: bool,
    pub timestamp: i64,
}

impl IdentityAttestation {
    pub fn is_expired(&self, ttl: i64) -> bool {
        false
    }
    pub fn verify_architect_signature(&self, verifier: &(dyn crate::attestation::AttestationVerifier + Send + Sync)) -> Result<bool, String> {
        Ok(true)
    }
}

pub trait IdentityAttestationProvider {
    fn attest_identity(
        &self,
        force_refresh: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<IdentityAttestation, String>> + Send + '_>>;
}

pub struct DummyIdentityProvider;
impl IdentityAttestationProvider for DummyIdentityProvider {
    fn attest_identity(
        &self,
        force_refresh: bool,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<IdentityAttestation, String>> + Send + '_>> {
        Box::pin(async {
            Ok(IdentityAttestation {
                confidence: 0.99,
                identity_verified: true,
                timestamp: 0,
            })
        })
    }
}
