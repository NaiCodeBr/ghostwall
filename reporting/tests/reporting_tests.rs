use ghostwall_reporting::ReportEngine;
use ghostwall_core::{Device, DeviceType, SecurityType, RiskScore, ExposureScore, ExposureMetrics};

#[test]
fn test_report_engine_creation() {
    let report_engine = ReportEngine::new();
    
    // Should create successfully
    assert!(true);
}

#[test]
fn test_report_generation() {
    let report_engine = ReportEngine::new();
    
    let devices = vec![
        Device::new(
            "AA:BB:CC:DD:EE:FF".to_string(),
            Some("Espressif".to_string()),
            Some("TestSSID".to_string()),
            Some("AA:BB:CC:DD:EE:FF".to_string()),
            -45,
            6,
            SecurityType::WPA2,
            DeviceType::ESP32,
        ),
    ];
    
    let risk_scores = vec![RiskScore::new()];
    let exposure_score = ExposureScore::new(ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    });
    
    let report = report_engine.generate_report(devices, risk_scores, exposure_score);
    
    assert!(!report.id.is_empty());
    assert_eq!(report.title, "GHOSTWALL RF Privacy Report");
    assert_eq!(report.inventory.len(), 1);
}

#[test]
fn test_report_export_json() {
    let report_engine = ReportEngine::new();
    
    let devices = vec![];
    let risk_scores = vec![];
    let exposure_score = ExposureScore::new(ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    });
    
    let report = report_engine.generate_report(devices, risk_scores, exposure_score);
    let json = report_engine.export_json(&report).unwrap();
    
    assert!(json.contains("GHOSTWALL RF Privacy Report"));
    assert!(json.contains("inventory"));
}

#[test]
fn test_report_export_html() {
    let report_engine = ReportEngine::new();
    
    let devices = vec![];
    let risk_scores = vec![];
    let exposure_score = ExposureScore::new(ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    });
    
    let report = report_engine.generate_report(devices, risk_scores, exposure_score);
    let html = report_engine.export_html(&report).unwrap();
    
    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("GHOSTWALL RF Privacy Report"));
}
