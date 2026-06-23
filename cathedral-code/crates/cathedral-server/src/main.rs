use std::sync::Arc;
use cathedral_server::api::create_routes;
use cathedral_server::orchestration::Orchestrator;
use cathedral_remix_bridge::client::RemixClient;
use cathedral_wormgraph::WormGraphClient;
use cathedral_zk::ZKGateway;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let remix = Arc::new(RemixClient::new("http://localhost:3000".to_string()));
    let wormgraph = Arc::new(WormGraphClient::new());
    let zk = Arc::new(ZKGateway::new());

    let orchestrator = Arc::new(Orchestrator::new(remix, wormgraph, zk));
    let app = create_routes(orchestrator);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on 8000");
    axum::serve(listener, app).await.unwrap();
}
