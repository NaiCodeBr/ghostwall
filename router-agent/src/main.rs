use ghostwall_router_agent::RouterAgent;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::info;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ghostwall_router_agent=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database path
    let db_path = PathBuf::from("/var/lib/ghostwall/ghostwall.db");
    
    // Create router agent
    let mut agent = RouterAgent::new(&db_path).await?;
    
    // Run with 60 second interval
    info!("Starting GHOSTWALL Router Agent");
    agent.run(60).await?;
    
    Ok(())
}
