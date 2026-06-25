pub mod auth;
pub mod compile;

use axum::{
    routing::{get, post},
    Router,
    middleware,
};
use std::sync::Arc;
use crate::orchestration::Orchestrator;
use crate::api::auth::did_auth_middleware;

pub fn create_routes(orchestrator: Arc<Orchestrator>) -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/compile", post(compile::compile_contract))
                .route("/health", get(health_check))
                .layer(middleware::from_fn(did_auth_middleware))
                .with_state(orchestrator),
        )
}

async fn health_check() -> &'static str {
    "OK"
}
