use ghostwall_scanner::Scanner;
use ghostwall_risk_engine::RiskEngine;
use ghostwall_storage::Storage;
use tracing::{info, error};

/// Router Agent - Deploy on routers for continuous RF monitoring
pub struct RouterAgent {
    scanner: Scanner,
    risk_engine: RiskEngine,
    storage: Storage,
}

impl RouterAgent {
    pub async fn new(db_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let scanner = Scanner::new()?;
        let risk_engine = RiskEngine::new();
        let storage = Storage::new(db_path).await?;
        
        Ok(Self {
            scanner,
            risk_engine,
            storage,
        })
    }

    /// Run continuous monitoring loop
    pub async fn run(&mut self, interval_seconds: u64) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting router agent with {}s interval", interval_seconds);
        
        loop {
            match self.scan_and_analyze().await {
                Ok(_) => info!("Scan completed successfully"),
                Err(e) => error!("Scan failed: {}", e),
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(interval_seconds)).await;
        }
    }

    async fn scan_and_analyze(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Scan for devices
        let devices = self.scanner.scan().await?;
        
        // Save devices to storage
        for device in &devices {
            self.storage.save_device(device).await?;
        }
        
        // Evaluate risk for each device
        for device in &devices {
            let risk_score = self.risk_engine.evaluate_device(device, None);
            self.storage.save_risk_score(&device.id.to_string(), &risk_score).await?;
        }
        
        Ok(())
    }
}
