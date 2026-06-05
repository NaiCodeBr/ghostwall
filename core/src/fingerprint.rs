use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FingerprintData {
    pub beacon_interval: Option<u32>,
    pub channel_utilization: Option<f64>,
    pub rssi_stability: Option<f64>,
    pub broadcast_frequency: Option<f64>,
    pub probe_requests: Option<u32>,
    pub udp_patterns: Vec<UdpPattern>,
    pub mqtt_telemetry: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpPattern {
    pub size: u32,
    pub frequency: f64,
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub id: String,
    pub device_id: String,
    pub data: FingerprintData,
    pub signature: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Fingerprint {
    pub fn new(device_id: String, data: FingerprintData) -> Self {
        let signature = Self::generate_signature(&data);
        
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            device_id,
            data,
            signature,
            created_at: chrono::Utc::now(),
        }
    }

    fn generate_signature(data: &FingerprintData) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        
        data.beacon_interval.hash(&mut hasher);
        data.channel_utilization.map(|v| (v * 100.0) as u64).hash(&mut hasher);
        data.rssi_stability.map(|v| (v * 100.0) as u64).hash(&mut hasher);
        data.broadcast_frequency.map(|v| (v * 100.0) as u64).hash(&mut hasher);
        data.probe_requests.hash(&mut hasher);
        data.mqtt_telemetry.hash(&mut hasher);
        
        for pattern in &data.udp_patterns {
            pattern.size.hash(&mut hasher);
            (pattern.frequency * 100.0) as u64.hash(&mut hasher);
        }
        
        format!("{:x}", hasher.finish())
    }

    pub fn is_sensing_compatible(&self) -> bool {
        // Indicators compatible with RF sensing architectures
        let continuous_operation = self.data.beacon_interval.is_some() 
            && self.data.beacon_interval.unwrap() < 100;
        
        let high_frequency_broadcasts = self.data.broadcast_frequency.is_some()
            && self.data.broadcast_frequency.unwrap() > 10.0;
        
        let frequent_small_udp = self.data.udp_patterns.iter()
            .any(|p| p.size < 100 && p.frequency > 5.0);
        
        let mqtt_enabled = self.data.mqtt_telemetry;
        
        continuous_operation || high_frequency_broadcasts || frequent_small_udp || mqtt_enabled
    }
}
