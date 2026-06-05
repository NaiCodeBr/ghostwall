use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceType {
    WiFi,
    ESP32,
    RaspberryPi,
    Broadcom,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityType {
    Open,
    WEP,
    WPA,
    WPA2,
    WPA3,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub mac_address: String,
    pub vendor: Option<String>,
    pub ssid: Option<String>,
    pub bssid: Option<String>,
    pub rssi: i32,
    pub channel: u32,
    pub security: SecurityType,
    pub device_type: DeviceType,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub is_active: bool,
    pub estimated_uptime_seconds: Option<u64>,
}

impl Device {
    pub fn new(
        mac_address: String,
        vendor: Option<String>,
        ssid: Option<String>,
        bssid: Option<String>,
        rssi: i32,
        channel: u32,
        security: SecurityType,
        device_type: DeviceType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            mac_address,
            vendor,
            ssid,
            bssid,
            rssi,
            channel,
            security,
            device_type,
            first_seen: now,
            last_seen: now,
            is_active: true,
            estimated_uptime_seconds: None,
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
        if let Some(uptime) = self.estimated_uptime_seconds {
            self.estimated_uptime_seconds = Some(
                uptime + (self.last_seen - self.first_seen).num_seconds() as u64
            );
        }
    }
}
