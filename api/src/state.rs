use ghostwall_scanner::Scanner;
use ghostwall_risk_engine::RiskEngine;
use ghostwall_storage::Storage;
use ghostwall_reporting::ReportEngine;
use ghostwall_threat_intel::ThreatDatabase;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Shared API state
pub struct ApiState {
    pub scanner: Arc<Mutex<Scanner>>,
    pub risk_engine: Arc<RiskEngine>,
    pub storage: Arc<Storage>,
    pub report_engine: Arc<ReportEngine>,
    pub threat_db: Arc<Mutex<ThreatDatabase>>,
}

impl ApiState {
    pub async fn new(database_path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let scanner = Arc::new(Mutex::new(Scanner::new()?));
        let risk_engine = Arc::new(RiskEngine::new());
        let storage = Arc::new(Storage::new(database_path).await?);
        let report_engine = Arc::new(ReportEngine::new());
        let threat_db = Arc::new(Mutex::new(ThreatDatabase::new()));
        
        Ok(Self {
            scanner,
            risk_engine,
            storage,
            report_engine,
            threat_db,
        })
    }
}
