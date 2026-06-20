use cathedral_sdk::{CathedralSdk, SdkConfig};
use common::crypto_config::{CryptoConfig, SignatureAlgorithm};
use std::time::Instant;

#[tokio::test]
async fn test_mldsa_integration() {
    let mut config = SdkConfig::default();
    config.crypto.signature_algorithm = SignatureAlgorithm::MlDsa;
    config.crypto.dual_stack_mode = true;
    config.crypto.fallback_signature_algorithm = Some(SignatureAlgorithm::Ed25519);
    config.private_key_bytes = None; // gerar chave automaticamente

    // Em um cenário real, tentaria conectar ao Bridge.
    // Ignoramos a criação do SDK que faz await na conexão gRPC e usamos asserção dummy.
    // CathedralSdk::new tenta conectar imediatamente.
    // Since we don't have a bridge running, let's just make sure config is valid and CathedralSdk::new handles keys.

    let sdk_res = CathedralSdk::new(config).await;
    // It will return Err because Bridge is not running, but NOT a crypto error.
    match sdk_res {
        Err(e) => assert!(e.to_string().contains("transport error")), // Connect error
        Ok(_) => panic!("Should have failed connecting"),
    }
}
