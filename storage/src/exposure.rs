use ghostwall_core::{Device, ExposureScore, ExposureMetrics, Result};
use tracing::{info, debug};

/// RF Exposure Scanner - Module 4
/// Evaluates the ease of RF sensing in the environment
pub struct ExposureScanner {
    coverage_threshold: f64,
}

impl ExposureScanner {
    pub fn new() -> Self {
        Self {
            coverage_threshold: 0.5,
        }
    }

    /// Calculate RF exposure score based on devices in the environment
    pub fn calculate_exposure(&self, devices: &[Device]) -> ExposureScore {
        info!("Calculating RF exposure score for {} devices", devices.len());
        
        let metrics = self.calculate_metrics(devices);
        let exposure_score = ExposureScore::new(metrics);
        
        debug!("RF Exposure Score: {}", exposure_score.score);
        
        exposure_score
    }

    fn calculate_metrics(&self, devices: &[Device]) -> ExposureMetrics {
        let device_count = devices.len() as u32;
        
        // Count unique APs (devices with SSID)
        let ap_count = devices.iter()
            .filter(|d| d.ssid.is_some())
            .collect::<std::collections::HashSet<_>>()
            .len() as u32;
        
        // Calculate WiFi coverage based on RSSI
        let wifi_coverage = self.calculate_coverage(devices);
        
        // Calculate RF density (devices per channel)
        let rf_density = self.calculate_rf_density(devices);
        
        // Calculate spatial distribution (channels used)
        let spatial_distribution = self.calculate_spatial_distribution(devices);
        
        ExposureMetrics {
            wifi_coverage,
            ap_count,
            device_count,
            rf_density,
            spatial_distribution,
        }
    }

    fn calculate_coverage(&self, devices: &[Device]) -> f64 {
        if devices.is_empty() {
            return 0.0;
        }
        
        // Count devices with good signal (RSSI > -70 dBm)
        let strong_signal_count = devices.iter()
            .filter(|d| d.rssi > -70)
            .count();
        
        (strong_signal_count as f64) / (devices.len() as f64)
    }

    fn calculate_rf_density(&self, devices: &[Device]) -> f64 {
        if devices.is_empty() {
            return 0.0;
        }
        
        // Calculate devices per channel
        let mut channel_counts: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for device in devices {
            *channel_counts.entry(device.channel).or_insert(0) += 1;
        }
        
        let max_devices_per_channel = channel_counts.values().cloned().max().unwrap_or(0);
        
        // Normalize to 0-1 range (assuming 10 devices per channel is high density)
        (max_devices_per_channel as f64 / 10.0).min(1.0)
    }

    fn calculate_spatial_distribution(&self, devices: &[Device]) -> f64 {
        if devices.is_empty() {
            return 0.0;
        }
        
        // Count unique channels used
        let unique_channels: std::collections::HashSet<u32> = devices.iter()
            .map(|d| d.channel)
            .collect();
        
        // More channels = better spatial distribution
        (unique_channels.len() as f64 / 14.0).min(1.0) // Assuming 14 channels max
    }
}

impl Default for ExposureScanner {
    fn default() -> Self {
        Self::new()
    }
}
