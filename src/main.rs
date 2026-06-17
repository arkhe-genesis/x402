use std::sync::Arc;
use tracing::{error, info};

pub mod attestation;
pub mod identity_attestation;
pub mod mcp;
pub mod voice;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configuração do MCP
    let mcp_enabled = std::env::var("ENABLE_MCP_SERVER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    if mcp_enabled {
        let mcp_port = std::env::var("MCP_PORT")
            .unwrap_or_else(|_| "3032".to_string())
            .parse::<u16>()
            .unwrap_or(3032);

        let mcp_token = std::env::var("MCP_AUTH_TOKEN").ok();

        // 2. Cria execution provider (CathedralComputeProvider)
        let execution_provider: Arc<dyn crate::attestation::AttestationProvider + Send + Sync> =
            Arc::new(crate::attestation::CathedralComputeProvider::new(
                "dummy".to_string(),
                "dummy".to_string(),
                "dummy".to_string(),
                "cathedral-v1",
            ));

        // 3. Verificador (pode ser opcional)
        let architect_verifier: Option<Arc<dyn crate::attestation::AttestationVerifier + Send + Sync>> = None;

        // 4. Inicia o servidor
        let attestation_manager_clone = Arc::new(crate::attestation::AttestationManager);
        let identity_provider_clone: Arc<dyn crate::identity_attestation::IdentityAttestationProvider + Send + Sync> = Arc::new(crate::identity_attestation::DummyIdentityProvider);
        let execution_provider_clone = execution_provider.clone();
        let voice_core_clone = Some(Arc::new(crate::voice::VoiceCore));

        tokio::spawn(async move {
            if let Err(e) = crate::mcp::server::start_mcp_server(
                attestation_manager_clone,
                identity_provider_clone,
                execution_provider_clone,
                architect_verifier,
                voice_core_clone,
                mcp_port,
            )
            .await
            {
                error!("❌ MCP Server falhou: {}", e);
            }
        });

        info!("🧠 MCP Server iniciado na porta {}", mcp_port);
    }

    Ok(())
}
