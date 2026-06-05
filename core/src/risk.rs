use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::Info => "info",
            RiskLevel::Low => "low",
            RiskLevel::Medium => "medium",
            RiskLevel::High => "high",
            RiskLevel::Critical => "critical",
        }
    }

    pub fn from_score(score: i32) -> Self {
        match score {
            0..=20 => RiskLevel::Info,
            21..=40 => RiskLevel::Low,
            41..=60 => RiskLevel::Medium,
            61..=80 => RiskLevel::High,
            81..=100 => RiskLevel::Critical,
            _ => RiskLevel::Critical,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub score: i32,
    pub conditions: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub total_score: i32,
    pub level: RiskLevel,
    pub applied_rules: Vec<RiskRule>,
    pub recommendations: Vec<String>,
}

impl RiskScore {
    pub fn new() -> Self {
        Self {
            total_score: 0,
            level: RiskLevel::Info,
            applied_rules: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: RiskRule) {
        self.total_score += rule.score;
        self.applied_rules.push(rule);
        self.level = RiskLevel::from_score(self.total_score);
    }

    pub fn add_recommendation(&mut self, recommendation: String) {
        self.recommendations.push(recommendation);
    }
}

impl Default for RiskScore {
    fn default() -> Self {
        Self::new()
    }
}
