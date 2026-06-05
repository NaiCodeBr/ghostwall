use ghostwall_core::{ExposureScore, ExposureMetrics};

#[test]
fn test_exposure_metrics_creation() {
    let metrics = ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    };
    
    assert_eq!(metrics.wifi_coverage, 0.8);
    assert_eq!(metrics.ap_count, 5);
    assert_eq!(metrics.device_count, 10);
}

#[test]
fn test_exposure_score_calculation() {
    let metrics = ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    };
    
    let exposure_score = ExposureScore::new(metrics);
    
    assert!(exposure_score.score >= 0);
    assert!(exposure_score.score <= 100);
}

#[test]
fn test_exposure_score_high_coverage() {
    let metrics = ExposureMetrics {
        wifi_coverage: 0.9,
        ap_count: 3,
        device_count: 5,
        rf_density: 0.3,
        spatial_distribution: 0.4,
    };
    
    let exposure_score = ExposureScore::new(metrics);
    
    assert!(!exposure_score.factors.is_empty());
    assert!(exposure_score.factors.iter().any(|f| f.contains("Alta cobertura")));
}

#[test]
fn test_exposure_score_serialization() {
    let metrics = ExposureMetrics {
        wifi_coverage: 0.8,
        ap_count: 5,
        device_count: 10,
        rf_density: 0.5,
        spatial_distribution: 0.6,
    };
    
    let exposure_score = ExposureScore::new(metrics);
    
    let serialized = serde_json::to_string(&exposure_score).unwrap();
    let deserialized: ExposureScore = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(exposure_score.score, deserialized.score);
    assert_eq!(exposure_score.metrics.wifi_coverage, deserialized.metrics.wifi_coverage);
}
