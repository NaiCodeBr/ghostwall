use axum::{
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use tracing::info;
use std::sync::Arc;

pub mod handlers;
pub mod state;

pub use state::ApiState;

/// Create the REST API router
pub fn create_router(state: Arc<ApiState>) -> Router {
    Router::new()
        .route("/", get(handlers::health_check))
        .route("/api/health", get(handlers::health_check))
        .route("/api/scan", post(handlers::start_scan))
        .route("/api/devices", get(handlers::get_devices))
        .route("/api/devices/:id", get(handlers::get_device))
        .route("/api/risk/evaluate", post(handlers::evaluate_risk))
        .route("/api/exposure", get(handlers::get_exposure))
        .route("/api/reports", post(handlers::generate_report))
        .route("/api/threat-intel", get(handlers::get_threat_intel))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        )
        .with_state(state)
}
