use ghostwall_risk_engine::RiskEngine;
use ghostwall_core::{Device, DeviceType, SecurityType};

#[test]
fn test_risk_engine_creation() {
    let risk_engine = RiskEngine::new();
    
    // Should have default rules loaded
    assert!(!risk_engine.rules.is_empty());
}

#[test]
fn test_risk_engine_evaluate_esp32() {
    let risk_engine = RiskEngine::new();
    
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
    
    let risk_score = risk_engine.evaluate_device(&device, None);
    
    // ESP32 should trigger GW-RF-001
    assert!(risk_score.total_score > 0);
}

#[test]
fn test_risk_engine_evaluate_cluster() {
    let risk_engine = RiskEngine::new();
    
    let devices = vec![
        Device::new(
            "AA:BB:CC:DD:EE:01".to_string(),
            Some("Espressif".to_string()),
            Some("TestSSID".to_string()),
            Some("AA:BB:CC:DD:EE:01".to_string()),
            -45,
            6,
            SecurityType::WPA2,
            DeviceType::ESP32,
        ),
        Device::new(
            "AA:BB:CC:DD:EE:02".to_string(),
            Some("Espressif".to_string()),
            Some("TestSSID".to_string()),
            Some("AA:BB:CC:DD:EE:02".to_string()),
            -45,
            6,
            SecurityType::WPA2,
            DeviceType::ESP32,
        ),
        Device::new(
            "AA:BB:CC:DD:EE:03".to_string(),
            Some("Espressif".to_string()),
            Some("TestSSID".to_string()),
            Some("AA:BB:CC:DD:EE:03".to_string()),
            -45,
            6,
            SecurityType::WPA2,
            DeviceType::ESP32,
        ),
    ];
    
    let risk_score = risk_engine.evaluate_cluster(&devices, &[]);
    
    // 3 ESP32 devices should trigger GW-RF-002
    assert!(risk_score.total_score >= 25);
}
