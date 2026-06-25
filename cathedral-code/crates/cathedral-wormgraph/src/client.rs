use cathedral_identity::Did;
use serde_json::Value;

pub struct WormGraphClient {}

impl WormGraphClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn record_action(
        &self,
        did: &Did,
        action: &str,
        data: Value,
    ) -> Result<String, String> {
        // Dummy implementation
        Ok("action_id_dummy".to_string())
    }
}
