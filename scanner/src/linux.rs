use ghostwall_core::{Device, DeviceType, SecurityType, Result, GhostwallError};
use tracing::{info, debug, warn};
use std::process::Command;

/// Linux RF scanner using iw and nl80211
pub struct LinuxScanner {
    interface: Option<String>,
}

impl LinuxScanner {
    pub fn new() -> Result<Self> {
        let interface = Self::find_wifi_interface();
        Ok(Self { interface })
    }

    fn find_wifi_interface() -> Option<String> {
        let output = Command::new("iw")
            .args(&["dev"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("Interface") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        return Some(parts[1].to_string());
                    }
                }
            }
        }
        None
    }

    pub async fn scan(&self) -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        
        if let Some(ref iface) = self.interface {
            info!("Scanning on interface: {}", iface);
            
            // Scan for WiFi networks
            match self.scan_wifi(iface).await {
                Ok(mut wifi_devices) => {
                    devices.append(&mut wifi_devices);
                }
                Err(e) => warn!("WiFi scan failed: {}", e),
            }
            
            // Scan for devices on the network
            match self.scan_network(iface).await {
                Ok(mut network_devices) => {
                    devices.append(&mut network_devices);
                }
                Err(e) => warn!("Network scan failed: {}", e),
            }
        } else {
            warn!("No WiFi interface found");
        }
        
        Ok(devices)
    }

    async fn scan_wifi(&self, interface: &str) -> Result<Vec<Device>> {
        let output = Command::new("iw")
            .args(&[interface, "scan"])
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    return Err(GhostwallError::ScanError(
                        "iw scan command failed".to_string()
                    ));
                }
                
                self.parse_iw_scan(&String::from_utf8_lossy(&output.stdout))
            }
            Err(e) => {
                Err(GhostwallError::ScanError(format!("Failed to run iw: {}", e)))
            }
        }
    }

    fn parse_iw_scan(&self, output: &str) -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        let mut current_device: Option<Device> = None;
        
        for line in output.lines() {
            let line = line.trim();
            
            if line.starts_with("BSS ") {
                if let Some(device) = current_device.take() {
                    devices.push(device);
                }
                
                if let Some(bssid) = line.split_whitespace().nth(1) {
                    let bssid_clean = bssid.trim_end_matches('(on');
                    let device = Device::new(
                        bssid_clean.to_string(),
                        None,
                        None,
                        Some(bssid_clean.to_string()),
                        -60, // Default RSSI
                        0,   // Default channel
                        SecurityType::Unknown,
                        DeviceType::WiFi,
                    );
                    current_device = Some(device);
                }
            } else if let Some(ref mut device) = current_device {
                if line.starts_with("SSID:") {
                    device.ssid = Some(line[5..].trim().to_string());
                } else if line.starts_with("signal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(rssi) = parts[1].parse::<i32>() {
                            device.rssi = rssi;
                        }
                    }
                } else if line.contains("DS Parameter set: channel") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if let Some(channel_str) = parts.last() {
                        if let Ok(channel) = channel_str.parse::<u32>() {
                            device.channel = channel;
                        }
                    }
                }
            }
        }
        
        if let Some(device) = current_device {
            devices.push(device);
        }
        
        Ok(devices)
    }

    async fn scan_network(&self, _interface: &str) -> Result<Vec<Device>> {
        // Implementation would use ARP scanning or similar
        // For now, return empty as this requires elevated privileges
        Ok(Vec::new())
    }
}
