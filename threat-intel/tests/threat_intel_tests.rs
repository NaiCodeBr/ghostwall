use ghostwall_threat_intel::ThreatDatabase;
use ghostwall_core::{Device, DeviceType, SecurityType, Fingerprint, FingerprintData, UdpPattern};

#[test]
fn test_threat_database_creation() {
    let threat_db = ThreatDatabase::new();
    
    assert!(!threat_db.signatures.is_empty());
    assert_eq!(threat_db.version(), "1.0.0");
}

#[test]
fn test_threat_database_search() {
    let threat_db = ThreatDatabase::new();
    
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
    
    let matches = threat_db.search(&device, None);
    
    // Should match Espressif signature
    assert!(!matches.is_empty());
}

#[test]
fn test_threat_database_add_signature() {
    let mut threat_db = ThreatDatabase::new();
    
    let initial_count = threat_db.signatures.len();
    
    let new_signature = ghostwall_threat_intel::ThreatSignature {
        id: "TEST-001".to_string(),
        vendor: "TestVendor".to_string(),
        fingerprint_pattern: "test.*pattern".to_string(),
        risk_score: 15,
        tags: vec!["test".to_string()],
        version: "1.0.0".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    threat_db.add_signature(new_signature);
    
    assert_eq!(threat_db.signatures.len(), initial_count + 1);
}

#[test]
fn test_threat_database_export() {
    let threat_db = ThreatDatabase::new();
    
    let exported = threat_db.export().unwrap();
    
    assert!(exported.contains("signatures"));
    assert!(exported.contains("version"));
}

#[test]
fn test_threat_database_import() {
    let mut threat_db = ThreatDatabase::new();
    
    let json = r#"[
        {
            "id": "TEST-002",
            "vendor": "TestVendor",
            "fingerprint_pattern": "test.*pattern",
            "risk_score": 20,
            "tags": ["test"],
            "version": "1.0.0",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]"#;
    
    threat_db.import(json).unwrap();
    
    assert!(threat_db.signatures.iter().any(|s| s.id == "TEST-002"));
}
