pub mod agents {
    pub mod llama_zip_agent;
}
pub mod agent_loop {
    pub struct AgentResult {
        pub final_answer: String,
        pub steps_taken: u32,
        pub tools_used: Vec<String>,
        pub latency_secs: f64,
        pub memory_consolidated: bool,
    }
    pub enum AgentError {
        ToolError(String),
    }
    impl From<String> for AgentError {
        fn from(s: String) -> Self {
            AgentError::ToolError(s)
        }
    }
    #[async_trait::async_trait]
    pub trait CathedralAgent {
        async fn run(&mut self, goal: &str) -> Result<AgentResult, AgentError>;
        fn id(&self) -> crate::orchestrator::AgentId;
    }
}
pub mod orchestrator {
    #[derive(Debug, Clone)]
    pub struct AgentId(String);
}
