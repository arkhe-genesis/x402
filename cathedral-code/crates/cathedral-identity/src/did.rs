use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Did {
    pub method: String,
    pub namespace: String,
    pub identifier: String,
    pub public_key: Vec<u8>,
}

impl Did {
    pub fn new(method: &str, namespace: &str, identifier: &str) -> Self {
        Self {
            method: method.to_string(),
            namespace: namespace.to_string(),
            identifier: identifier.to_string(),
            public_key: vec![],
        }
    }

    pub fn parse(did_str: &str) -> Result<Self, String> {
        let parts: Vec<&str> = did_str.split(':').collect();
        if parts.len() < 4 || parts[0] != "did" {
            return Err("Invalid DID format".to_string());
        }
        Ok(Self {
            method: parts[1].to_string(),
            namespace: parts[2].to_string(),
            identifier: parts[3..].join(":"),
            public_key: vec![],
        })
    }

    pub fn to_string(&self) -> String {
        format!("did:{}:{}:{}", self.method, self.namespace, self.identifier)
    }
}

pub fn verify_signature(did: &Did, signature: &[u8], message: &[u8]) -> bool {
    // Dummy implementation for now
    true
}
