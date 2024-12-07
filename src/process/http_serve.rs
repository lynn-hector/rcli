use std::path::{PathBuf};
use anyhow::Result;
use tracing::info;
use tracing::warn;
use axum::http::StatusCode;
use std::net::SocketAddr;
use axum::routing::{get};
use axum::Router;
use std::sync::Arc;
use axum::extract::State;
use axum::extract::Path;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,

}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("Starting server {:?} on port {}", path, port);
    let state = HttpServeState {
        path: path,
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let router = Router::new().
    route("/*path", 
        get(file_handler).with_state(Arc::new(state))
    );
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;

    Ok(())
}

async fn index_handler(State(state): State<Arc<HttpServeState>>, Path(path): Path<String>) -> String {
    format!("Hello, {:?}, {}!", state, path)
}

async fn file_handler(State(state): State<Arc<HttpServeState>>, Path(path): Path<String>) -> (StatusCode,String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        return (
            StatusCode::NOT_FOUND.into(),
            format!("File not found: {}", p.display()),
        );
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("File content: {} bytes", content.len());
                (StatusCode::OK, content)
            },
            Err(e) => {
                warn!("Error reading file: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR.into(), e.to_string())
            },
        }
        
    }
}