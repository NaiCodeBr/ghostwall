use ghostwall_core::{RiskScore, RiskLevel, RiskRule};
use std::collections::HashMap;

#[test]
fn test_risk_score_creation() {
    let risk_score = RiskScore::new();
    
    assert_eq!(risk_score.total_score, 0);
    assert_eq!(risk_score.level, RiskLevel::Info);
    assert!(risk_score.applied_rules.is_empty());
    assert!(risk_score.recommendations.is_empty());
}

#[test]
fn test_risk_score_add_rule() {
    let mut risk_score = RiskScore::new();
    
    let rule = RiskRule {
        id: "TEST-001".to_string(),
        name: "Test Rule".to_string(),
        description: "Test description".to_string(),
        score: 25,
        conditions: HashMap::new(),
    };
    
    risk_score.add_rule(rule);
    
    assert_eq!(risk_score.total_score, 25);
    assert_eq!(risk_score.level, RiskLevel::Low);
    assert_eq!(risk_score.applied_rules.len(), 1);
}

#[test]
fn test_risk_level_from_score() {
    assert_eq!(RiskLevel::from_score(0), RiskLevel::Info);
    assert_eq!(RiskLevel::from_score(20), RiskLevel::Info);
    assert_eq!(RiskLevel::from_score(25), RiskLevel::Low);
    assert_eq!(RiskLevel::from_score(45), RiskLevel::Medium);
    assert_eq!(RiskLevel::from_score(65), RiskLevel::High);
    assert_eq!(RiskLevel::from_score(85), RiskLevel::Critical);
}

#[test]
fn test_risk_score_add_recommendation() {
    let mut risk_score = RiskScore::new();
    
    risk_score.add_recommendation("Test recommendation".to_string());
    
    assert_eq!(risk_score.recommendations.len(), 1);
    assert_eq!(risk_score.recommendations[0], "Test recommendation");
}

#[test]
fn test_risk_score_serialization() {
    let mut risk_score = RiskScore::new();
    
    let rule = RiskRule {
        id: "TEST-001".to_string(),
        name: "Test Rule".to_string(),
        description: "Test description".to_string(),
        score: 25,
        conditions: HashMap::new(),
    };
    
    risk_score.add_rule(rule);
    risk_score.add_recommendation("Test recommendation".to_string());
    
    let serialized = serde_json::to_string(&risk_score).unwrap();
    let deserialized: RiskScore = serde_json::from_str(&serialized).unwrap();
    
    assert_eq!(risk_score.total_score, deserialized.total_score);
    assert_eq!(risk_score.level, deserialized.level);
}
