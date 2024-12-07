use std::path::Path;
use anyhow::Result;
use tracing::info;
use std::net::SocketAddr;
use axum::routing::{get};
use axum::Router;

pub async fn process_http_serve(path: &Path, port: u16) -> Result<()> {
    info!("Starting server {:?} on port {}", path, port);
    let router = Router::new().route("/", get(index_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index_handler() -> &'static str {
    "Hello, world!"
}