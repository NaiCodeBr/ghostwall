use ghostwall_core::{Device, DeviceType, SecurityType, Result};
use std::collections::HashMap;
use tracing::{info, warn, error};

pub mod linux;
pub mod macos;
pub mod windows;
pub mod fingerprint;
pub mod discovery;

pub use discovery::DiscoveryEngine;
pub use fingerprint::FingerprintEngine;

/// Main scanner interface
pub struct Scanner {
    discovery: DiscoveryEngine,
    fingerprint: FingerprintEngine,
}

impl Scanner {
    pub fn new() -> Result<Self> {
        Ok(Self {
            discovery: DiscoveryEngine::new()?,
            fingerprint: FingerprintEngine::new(),
        })
    }

    /// Scan for RF devices in the environment
    pub async fn scan(&mut self) -> Result<Vec<Device>> {
        info!("Starting RF device scan");
        let devices = self.discovery.discover_devices().await?;
        info!("Discovered {} devices", devices.len());
        Ok(devices)
    }

    /// Generate fingerprints for devices
    pub async fn generate_fingerprints(&mut self, devices: &[Device]) -> Result<Vec<ghostwall_core::Fingerprint>> {
        let mut fingerprints = Vec::new();
        for device in devices {
            match self.fingerprint.analyze_device(device).await {
                Ok(fp) => fingerprints.push(fp),
                Err(e) => warn!("Failed to fingerprint device {}: {}", device.id, e),
            }
        }
        Ok(fingerprints)
    }

    /// Continuous monitoring mode
    pub async fn start_monitoring(&mut self, callback: impl Fn(Vec<Device>) + Send + 'static) -> Result<()> {
        info!("Starting continuous RF monitoring");
        // Implementation would use async task for continuous scanning
        Ok(())
    }
}
