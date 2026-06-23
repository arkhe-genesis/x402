use serde::{Deserialize, Serialize};
use cathedral_identity::Did;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    Allowed,
    Restricted,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionEntry {
    pub operation: String,
    pub level: PermissionLevel,
    pub scope: Option<String>,
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissions {
    pub agent_did: Did,
    pub operations: Vec<PermissionEntry>,
    pub signature: Vec<u8>,
}

impl AgentPermissions {
    pub fn verify(&self) -> Result<bool, String> {
        Ok(cathedral_identity::verify_signature(
            &self.agent_did,
            &self.signature,
            &serde_json::to_vec(self).unwrap_or_default(),
        ))
    }

    pub fn check(&self, operation: &str, _scope: &str) -> Result<(), String> {
        for entry in &self.operations {
            if entry.operation == operation {
                return match entry.level {
                    PermissionLevel::Allowed => Ok(()),
                    PermissionLevel::Restricted => Ok(()), // Assume confirmed for now
                    PermissionLevel::Denied => Err(format!("Operation {} denied", operation)),
                };
            }
        }
        Err(format!("Operation {} not found in permissions", operation))
    }
}
