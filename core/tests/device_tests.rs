use ghostwall_core::{Device, DeviceType, SecurityType};
use chrono::Utc;

#[test]
fn test_device_creation() {
    let device = Device::new(
        "AA:BB:CC:DD:EE:FF".to_string(),
        Some("Espressif".to_string()),
        Some("TestSSID".to_string()),
        Some("AA:BB:CC:DD:EE:FF".to_string()),
        -45,
        6,
        SecurityType::WPA2,
        DeviceType::ESP32,
    );

    assert_eq!(device.mac_address, "AA:BB:CC:DD:EE:FF");
    assert_eq!(device.vendor, Some("Espressif".to_string()));
    assert_eq!(device.ssid, Some("TestSSID".to_string()));
    assert_eq!(device.rssi, -45);
    assert_eq!(device.channel, 6);
    assert_eq!(device.device_type, DeviceType::ESP32);
    assert!(device.is_active);
}

#[test]
fn test_device_update_last_seen() {
    let mut device = Device::new(
        "AA:BB:CC:DD:EE:FF".to_string(),
        Some("Espressif".to_string()),
        Some("TestSSID".to_string()),
        Some("AA:BB:CC:DD:EE:FF".to_string()),
        -45,
        6,
        SecurityType::WPA2,
        DeviceType::ESP32,
    );

    let original_last_seen = device.last_seen;
    std::thread::sleep(std::time::Duration::from_millis(10));
    device.update_last_seen();

    assert!(device.last_seen > original_last_seen);
}

#[test]
fn test_device_serialization() {
    let device = Device::new(
        "AA:BB:CC:DD:EE:FF".to_string(),
        Some("Espressif".to_string()),
        Some("TestSSID".to_string()),
        Some("AA:BB:CC:DD:EE:FF".to_string()),
        -45,
        6,
        SecurityType::WPA2,
        DeviceType::ESP32,
    );

    let serialized = serde_json::to_string(&device).unwrap();
    let deserialized: Device = serde_json::from_str(&serialized).unwrap();

    assert_eq!(device.mac_address, deserialized.mac_address);
    assert_eq!(device.vendor, deserialized.vendor);
}
