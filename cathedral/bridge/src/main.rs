pub mod grpc_service;
pub mod signature_verifier;

use common::crypto_config::crypto_config_from_env;
use signature_verifier::{PublicKeyRegistry, SignatureVerifier};
use std::sync::Arc;

use grpc_service::pb::cathedral_bridge_server::CathedralBridgeServer;
use grpc_service::MyCathedralBridge;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let crypto_config = crypto_config_from_env();
    println!("Configuração criptográfica: {:?}", crypto_config);

    let registry = Arc::new(PublicKeyRegistry::new(crypto_config.clone()));
    let verifier = Arc::new(SignatureVerifier::new(
        registry.clone(),
        crypto_config.clone(),
    ));
    let bridge = MyCathedralBridge { verifier };

    println!("CathedralBridge listening on {}", addr);

    Server::builder()
        .add_service(CathedralBridgeServer::new(bridge))
        .serve(addr)
        .await?;

    Ok(())
}
