use anyhow::Result;
use axum::{
    body::Body, extract::State, http::header, http::StatusCode, response::IntoResponse,
    routing::get, Json, Router,
};
use azure_storage_offload_core::metrics::Metrics;
use azure_storage_offload_core::{metrics, tracing as core_tracing};
use prometheus::{Encoder, TextEncoder};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::{error, info};

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[derive(Clone)]
struct AppState {
    metrics: &'static Metrics,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _guard = core_tracing::init_tracing("admin_api")?;
    let metrics = metrics::metrics();

    let state = AppState { metrics };
    let app = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:9090".parse()?;
    info!(%addr, "Starting admin API server");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.metrics.gather() {
        Ok(body) => {
            let mut response = axum::response::Response::new(Body::from(body));
            let encoder = TextEncoder::new();
            let format = encoder.format_type().to_string();
            if let Ok(value) = header::HeaderValue::from_str(&format) {
                response.headers_mut().insert(header::CONTENT_TYPE, value);
            }
            response
        }
        Err(err) => {
            error!(?err, "failed to gather metrics");
            let mut response = axum::response::Response::new(Body::from("metrics unavailable"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    }
}
