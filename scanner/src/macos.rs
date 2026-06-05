use ghostwall_core::{Device, DeviceType, SecurityType, Result, GhostwallError};
use tracing::{info, debug, warn};
use std::process::Command;

/// macOS RF scanner using CoreWLAN
pub struct MacOsScanner {
    interface: Option<String>,
}

impl MacOsScanner {
    pub fn new() -> Result<Self> {
        let interface = Self::find_wifi_interface();
        Ok(Self { interface })
    }

    fn find_wifi_interface() -> Option<String> {
        let output = Command::new("networksetup")
            .args(&["-listallhardwareports"])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut found_wifi = false;
            for line in stdout.lines() {
                if line.contains("Wi-Fi") || line.contains("AirPort") {
                    found_wifi = true;
                } else if found_wifi && line.trim().starts_with("Device:") {
                    let device = line.split(':').nth(1)?.trim().to_string();
                    return Some(device);
                }
            }
        }
        None
    }

    pub async fn scan(&self) -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        
        if let Some(ref iface) = self.interface {
            info!("Scanning on interface: {}", iface);
            
            match self.scan_wifi(iface).await {
                Ok(mut wifi_devices) => {
                    devices.append(&mut wifi_devices);
                }
                Err(e) => warn!("WiFi scan failed: {}", e),
            }
        } else {
            warn!("No WiFi interface found");
        }
        
        Ok(devices)
    }

    async fn scan_wifi(&self, interface: &str) -> Result<Vec<Device>> {
        let output = Command::new("/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport")
            .args(&["-s"])
            .output();
        
        match output {
            Ok(output) => {
                if !output.status.success() {
                    return Err(GhostwallError::ScanError(
                        "airport scan command failed".to_string()
                    ));
                }
                
                self.parse_airport_scan(&String::from_utf8_lossy(&output.stdout))
            }
            Err(e) => {
                Err(GhostwallError::ScanError(format!("Failed to run airport: {}", e)))
            }
        }
    }

    fn parse_airport_scan(&self, output: &str) -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        let lines: Vec<&str> = output.lines().collect();
        
        // Skip header line
        for line in lines.iter().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            
            if parts.len() >= 6 {
                let bssid = parts[0].to_string();
                let rssi = parts[2].parse::<i32>().unwrap_or(-60);
                let channel = parts[3].parse::<u32>().unwrap_or(0);
                let ssid = if parts.len() > 6 {
                    Some(parts[6..].join(" "))
                } else {
                    None
                };
                
                let device = Device::new(
                    bssid.clone(),
                    None,
                    ssid,
                    Some(bssid),
                    rssi,
                    channel,
                    SecurityType::Unknown,
                    DeviceType::WiFi,
                );
                
                devices.push(device);
            }
        }
        
        Ok(devices)
    }
}
