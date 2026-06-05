use axum::Router;
use ghostwall_api::create_router;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ghostwall_api=debug,tower_http=debug,axum=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database path
    let db_path = PathBuf::from("ghostwall.db");
    
    // Create API state
    let state = ghostwall_api::ApiState::new(&db_path).await?;
    let state = Arc::new(state);
    
    // Create router
    let app = create_router(state);
    
    // Start server
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    info!("GHOSTWALL API server listening on http://0.0.0.0:8080");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
