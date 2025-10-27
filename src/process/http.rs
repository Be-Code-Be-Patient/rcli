use crate::HttpServeOpts;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::read_to_string;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(opts: HttpServeOpts) -> anyhow::Result<()> {
    info!("Serve: {:?}", opts);
    let state = HttpServeState {
        path: opts.dir.clone(),
    };
    let router = Router::new()
        .route("/{*path}", get(file_handler))
        .nest_service(
            "/tower",
            ServeDir::new(opts.dir)
                .append_index_html_on_directories(true)
                .precompressed_gzip()
                .precompressed_br()
                .precompressed_deflate()
                .precompressed_zstd(),
        )
        .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], opts.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path_buf = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", path_buf);
    if !path_buf.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", path_buf.display()),
        )
    } else {
        match read_to_string(path_buf).await {
            Ok(content) => {
                info!("Read content success: {}", content);
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
