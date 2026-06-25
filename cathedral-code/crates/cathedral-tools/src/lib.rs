use async_trait::async_trait;
use cathedral_identity::Did;
use cathedral_wormgraph::WormGraphClient;
use cathedral_permissions::PermissionEntry;
use std::sync::Arc;
use serde_json::Value;

pub type ToolResult = Value;
pub type ToolParams = Value;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn permissions(&self) -> Vec<PermissionEntry>;
    async fn execute(&self, params: &ToolParams, context: &ToolContext) -> Result<ToolResult, String>;
}

pub struct ToolContext {
    pub agent_did: Did,
    pub session_id: String,
    pub wormgraph: Arc<WormGraphClient>,
}

impl ToolContext {
    pub async fn record_action(&self, action: &str, params: &ToolParams, result: &ToolResult) -> Result<(), String> {
        self.wormgraph.record_action(
            &self.agent_did,
            action,
            serde_json::json!({
                "params": params,
                "result": result,
                "session": self.session_id,
            }),
        ).await?;
        Ok(())
    }
}
