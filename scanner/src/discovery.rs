use ghostwall_core::{Device, DeviceType, SecurityType, Result, GhostwallError};
use std::collections::HashMap;
use tracing::{info, debug};

/// RF Discovery Engine - Module 1
/// Discovers RF devices in the network and environment
pub struct DiscoveryEngine {
    vendor_cache: HashMap<String, String>,
}

impl DiscoveryEngine {
    pub fn new() -> Result<Self> {
        let mut vendor_cache = HashMap::new();
        
        // Common vendor OUI prefixes
        vendor_cache.insert("AC:1B:3C".to_string(), "Espressif".to_string());
        vendor_cache.insert("DC:4F:22".to_string(), "Espressif".to_string());
        vendor_cache.insert("E8:50:8B".to_string(), "Espressif".to_string());
        vendor_cache.insert("B8:27:EB".to_string(), "Raspberry Pi".to_string());
        vendor_cache.insert("DC:A6:32".to_string(), "Raspberry Pi".to_string());
        vendor_cache.insert("E4:5F:01".to_string(), "Raspberry Pi".to_string());
        vendor_cache.insert("00:0C:43".to_string(), "Broadcom".to_string());
        vendor_cache.insert("00:11:22".to_string(), "Broadcom".to_string());
        vendor_cache.insert("00:90:4C".to_string(), "Broadcom".to_string());
        
        Ok(Self { vendor_cache })
    }

    /// Discover all RF devices in the environment
    pub async fn discover_devices(&self) -> Result<Vec<Device>> {
        let mut devices = Vec::new();
        
        #[cfg(target_os = "linux")]
        {
            devices.extend(self.discover_linux().await?);
        }
        
        #[cfg(target_os = "macos")]
        {
            devices.extend(self.discover_macos().await?);
        }
        
        #[cfg(target_os = "windows")]
        {
            devices.extend(self.discover_windows().await?);
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            return Err(GhostwallError::ScanError(
                "Unsupported platform".to_string()
            ));
        }
        
        Ok(devices)
    }

    #[cfg(target_os = "linux")]
    async fn discover_linux(&self) -> Result<Vec<Device>> {
        use crate::linux::LinuxScanner;
        let scanner = LinuxScanner::new()?;
        scanner.scan().await
    }

    #[cfg(target_os = "macos")]
    async fn discover_macos(&self) -> Result<Vec<Device>> {
        use crate::macos::MacOsScanner;
        let scanner = MacOsScanner::new()?;
        scanner.scan().await
    }

    #[cfg(target_os = "windows")]
    async fn discover_windows(&self) -> Result<Vec<Device>> {
        use crate::windows::WindowsScanner;
        let scanner = WindowsScanner::new()?;
        scanner.scan().await
    }

    /// Identify device type based on MAC and characteristics
    pub fn identify_device_type(&self, mac: &str, vendor: &Option<String>) -> DeviceType {
        if let Some(v) = vendor {
            if v.contains("Espressif") {
                return DeviceType::ESP32;
            }
            if v.contains("Raspberry") {
                return DeviceType::RaspberryPi;
            }
            if v.contains("Broadcom") {
                return DeviceType::Broadcom;
            }
        }
        
        // Check MAC OUI
        let oui = &mac[..8];
        if let Some(vendor) = self.vendor_cache.get(oui) {
            match vendor.as_str() {
                "Espressif" => DeviceType::ESP32,
                "Raspberry Pi" => DeviceType::RaspberryPi,
                "Broadcom" => DeviceType::Broadcom,
                _ => DeviceType::WiFi,
            }
        } else {
            DeviceType::WiFi
        }
    }

    /// Get vendor from MAC address
    pub fn get_vendor(&self, mac: &str) -> Option<String> {
        let oui = &mac[..8];
        self.vendor_cache.get(oui).cloned()
    }
}
