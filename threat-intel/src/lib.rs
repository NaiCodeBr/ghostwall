use ghostwall_core::{Device, Fingerprint, RiskScore, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};

/// Threat Intelligence - Module 5
/// Community database for RF threat signatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSignature {
    pub id: String,
    pub vendor: String,
    pub fingerprint_pattern: String,
    pub risk_score: i32,
    pub tags: Vec<String>,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDatabase {
    signatures: Vec<ThreatSignature>,
    version: String,
    last_updated: chrono::DateTime<chrono::Utc>,
}

impl ThreatDatabase {
    pub fn new() -> Self {
        let signatures = Self::load_default_signatures();
        
        Self {
            signatures,
            version: "1.0.0".to_string(),
            last_updated: chrono::Utc::now(),
        }
    }

    /// Load default threat signatures
    fn load_default_signatures() -> Vec<ThreatSignature> {
        vec![
            ThreatSignature {
                id: "THREAT-001".to_string(),
                vendor: "Espressif".to_string(),
                fingerprint_pattern: "ESP32-S3.*continuous.*mqtt".to_string(),
                risk_score: 30,
                tags: vec!["csi-compatible".to_string(), "iot".to_string()],
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            ThreatSignature {
                id: "THREAT-002".to_string(),
                vendor: "Broadcom".to_string(),
                fingerprint_pattern: "Nexmon.*monitor".to_string(),
                risk_score: 25,
                tags: vec!["nexmon".to_string(), "monitoring".to_string()],
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
            ThreatSignature {
                id: "THREAT-003".to_string(),
                vendor: "Raspberry Pi".to_string(),
                fingerprint_pattern: "monitor.*high-frequency".to_string(),
                risk_score: 20,
                tags: vec!["sensing".to_string(), "linux".to_string()],
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            },
        ]
    }

    /// Search for signatures matching a device
    pub fn search(&self, device: &Device, fingerprint: Option<&Fingerprint>) -> Vec<&ThreatSignature> {
        let mut matches = Vec::new();
        
        for signature in &self.signatures {
            // Match by vendor
            if let Some(vendor) = &device.vendor {
                if vendor.contains(&signature.vendor) {
                    matches.push(signature);
                    continue;
                }
            }
            
            // Match by fingerprint if available
            if let Some(fp) = fingerprint {
                let fp_str = serde_json::to_string(&fp.data).unwrap_or_default();
                if fp_str.contains(&signature.fingerprint_pattern) {
                    matches.push(signature);
                }
            }
        }
        
        matches
    }

    /// Get signature by ID
    pub fn get_signature(&self, id: &str) -> Option<&ThreatSignature> {
        self.signatures.iter().find(|s| s.id == id)
    }

    /// Add a new signature
    pub fn add_signature(&mut self, signature: ThreatSignature) {
        self.signatures.push(signature);
        self.last_updated = chrono::Utc::now();
    }

    /// Update signatures from remote source
    pub async fn update_from_remote(&mut self, url: &str) -> Result<()> {
        info!("Updating threat intelligence from: {}", url);
        
        let response = reqwest::get(url).await?;
        
        if response.status().is_success() {
            let new_signatures: Vec<ThreatSignature> = response.json().await?;
            
            // Merge signatures
            for new_sig in new_signatures {
                if let Some(existing) = self.signatures.iter_mut().find(|s| s.id == new_sig.id) {
                    *existing = new_sig;
                } else {
                    self.signatures.push(new_sig);
                }
            }
            
            self.last_updated = chrono::Utc::now();
            info!("Threat intelligence updated successfully");
        }
        
        Ok(())
    }

    /// Export signatures to JSON
    pub fn export(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.signatures)?)
    }

    /// Import signatures from JSON
    pub fn import(&mut self, json: &str) -> Result<()> {
        let new_signatures: Vec<ThreatSignature> = serde_json::from_str(json)?;
        
        for new_sig in new_signatures {
            if let Some(existing) = self.signatures.iter_mut().find(|s| s.id == new_sig.id) {
                *existing = new_sig;
            } else {
                self.signatures.push(new_sig);
            }
        }
        
        self.last_updated = chrono::Utc::now();
        
        Ok(())
    }

    /// Get database version
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get last update time
    pub fn last_updated(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_updated
    }
}

impl Default for ThreatDatabase {
    fn default() -> Self {
        Self::new()
    }
}
