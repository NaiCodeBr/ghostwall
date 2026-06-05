use ghostwall_core::{Device, Fingerprint, RiskScore, RiskRule, RiskLevel, DeviceType};
use std::collections::HashMap;
use tracing::{info, debug};

/// RSIE Engine - RF Sensing Indicators of Exposure
/// Module 3: Risk scoring engine based on RF sensing indicators
pub struct RiskEngine {
    rules: Vec<RiskRule>,
    known_safe_devices: Vec<String>,
}

impl RiskEngine {
    pub fn new() -> Self {
        let rules = Self::load_default_rules();
        let known_safe_devices = Self::load_known_safe_devices();
        
        Self {
            rules,
            known_safe_devices,
        }
    }

    /// Load default RSIE rules
    fn load_default_rules() -> Vec<RiskRule> {
        vec![
            // GW-RF-001: Hardware compatível com CSI
            RiskRule {
                id: "GW-RF-001".to_string(),
                name: "Hardware compatível com CSI".to_string(),
                description: "Detectado hardware compatível com Channel State Information (CSI)".to_string(),
                score: 10,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("device_type".to_string(), serde_json::json!(["ESP32-S3", "ESP32-C6", "Broadcom"]));
                    map
                },
            },
            
            // GW-RF-002: 3 ou mais dispositivos Espressif
            RiskRule {
                id: "GW-RF-002".to_string(),
                name: "Múltiplos dispositivos Espressif".to_string(),
                description: "Detectado cluster de 3 ou mais dispositivos Espressif".to_string(),
                score: 25,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("espressif_count".to_string(), serde_json::json!(3));
                    map
                },
            },
            
            // GW-RF-003: Dispositivo operando continuamente
            RiskRule {
                id: "GW-RF-003".to_string(),
                name: "Operação contínua".to_string(),
                description: "Dispositivo operando continuamente por período prolongado".to_string(),
                score: 15,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("uptime_hours".to_string(), serde_json::json!(24));
                    map
                },
            },
            
            // GW-RF-004: Fluxos UDP pequenos e frequentes
            RiskRule {
                id: "GW-RF-004".to_string(),
                name: "Padrão UDP suspeito".to_string(),
                description: "Detectados fluxos UDP pequenos e frequentes compatíveis com sensoriamento".to_string(),
                score: 20,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("udp_small_frequent".to_string(), serde_json::json!(true));
                    map
                },
            },
            
            // GW-RF-005: Telemetria MQTT
            RiskRule {
                id: "GW-RF-005".to_string(),
                name: "Telemetria MQTT detectada".to_string(),
                description: "Detectado tráfego MQTT indicando possível telemetria".to_string(),
                score: 10,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("mqtt_telemetry".to_string(), serde_json::json!(true));
                    map
                },
            },
            
            // GW-RF-010: Cluster de sensoriamento
            RiskRule {
                id: "GW-RF-010".to_string(),
                name: "Cluster de sensoriamento RF".to_string(),
                description: "Detectado cluster compatível com infraestrutura de sensoriamento RF".to_string(),
                score: 50,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("esp32_count".to_string(), serde_json::json!(3));
                    map.insert("has_mqtt_or_udp".to_string(), serde_json::json!(true));
                    map.insert("continuous_operation".to_string(), serde_json::json!(true));
                    map
                },
            },
            
            // GW-FP-001: Redução de falso positivo
            RiskRule {
                id: "GW-FP-001".to_string(),
                name: "Equipamento conhecido".to_string(),
                description: "Equipamento conhecido de baixo risco (tomadas, lâmpadas, impressoras, câmeras)".to_string(),
                score: -20,
                conditions: {
                    let mut map = HashMap::new();
                    map.insert("known_safe".to_string(), serde_json::json!(true));
                    map
                },
            },
        ]
    }

    /// Load known safe device patterns
    fn load_known_safe_devices() -> Vec<String> {
        vec![
            "smart_plug".to_string(),
            "smart_bulb".to_string(),
            "printer".to_string(),
            "camera".to_string(),
        ]
    }

    /// Evaluate risk for a single device
    pub fn evaluate_device(&self, device: &Device, fingerprint: Option<&Fingerprint>) -> RiskScore {
        let mut risk_score = RiskScore::new();
        
        // Check device type rules
        self.evaluate_device_type(device, &mut risk_score);
        
        // Check fingerprint rules if available
        if let Some(fp) = fingerprint {
            self.evaluate_fingerprint(fp, &mut risk_score);
        }
        
        // Check known safe devices
        self.evaluate_known_safe(device, &mut risk_score);
        
        // Generate recommendations
        self.generate_recommendations(&risk_score, device);
        
        risk_score
    }

    /// Evaluate risk for multiple devices (cluster analysis)
    pub fn evaluate_cluster(&self, devices: &[Device], fingerprints: &[Fingerprint]) -> RiskScore {
        let mut risk_score = RiskScore::new();
        
        // Count device types
        let espressif_count = devices.iter()
            .filter(|d| d.device_type == DeviceType::ESP32)
            .count();
        
        // Check cluster rules
        if espressif_count >= 3 {
            let has_mqtt_or_udp = fingerprints.iter()
                .any(|fp| fp.data.mqtt_telemetry || !fp.data.udp_patterns.is_empty());
            
            let continuous_operation = devices.iter()
                .any(|d| d.estimated_uptime_seconds.unwrap_or(0) > 86400); // 24 hours
            
            if has_mqtt_or_udp && continuous_operation {
                if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-010") {
                    risk_score.add_rule(rule.clone());
                }
            } else if espressif_count >= 3 {
                if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-002") {
                    risk_score.add_rule(rule.clone());
                }
            }
        }
        
        risk_score
    }

    fn evaluate_device_type(&self, device: &Device, risk_score: &mut RiskScore) {
        match device.device_type {
            DeviceType::ESP32 => {
                if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-001") {
                    risk_score.add_rule(rule.clone());
                }
            }
            DeviceType::Broadcom => {
                if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-001") {
                    risk_score.add_rule(rule.clone());
                }
            }
            _ => {}
        }
        
        // Check continuous operation
        if let Some(uptime) = device.estimated_uptime_seconds {
            if uptime > 86400 { // 24 hours
                if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-003") {
                    risk_score.add_rule(rule.clone());
                }
            }
        }
    }

    fn evaluate_fingerprint(&self, fingerprint: &Fingerprint, risk_score: &mut RiskScore) {
        // Check MQTT telemetry
        if fingerprint.data.mqtt_telemetry {
            if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-005") {
                risk_score.add_rule(rule.clone());
            }
        }
        
        // Check UDP patterns
        let small_frequent_udp = fingerprint.data.udp_patterns.iter()
            .any(|p| p.size < 100 && p.frequency > 5.0);
        
        if small_frequent_udp {
            if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-RF-004") {
                risk_score.add_rule(rule.clone());
            }
        }
    }

    fn evaluate_known_safe(&self, device: &Device, risk_score: &mut RiskScore) {
        // Check if device matches known safe patterns
        if let Some(vendor) = &device.vendor {
            for safe_pattern in &self.known_safe_devices {
                if vendor.to_lowercase().contains(safe_pattern) {
                    if let Some(rule) = self.rules.iter().find(|r| r.id == "GW-FP-001") {
                        risk_score.add_rule(rule.clone());
                    }
                    break;
                }
            }
        }
    }

    fn generate_recommendations(&self, risk_score: &RiskScore, device: &Device) {
        if risk_score.total_score >= 50 {
            // High risk recommendations
            if device.device_type == DeviceType::ESP32 {
                risk_score.recommendations.push(
                    "Realizar inspeção física do dispositivo Espressif".to_string()
                );
            }
            
            if risk_score.applied_rules.iter().any(|r| r.id == "GW-RF-010") {
                risk_score.recommendations.push(
                    "Cluster de sensoriamento detectado. Investigar infraestrutura completa.".to_string()
                );
            }
        }
        
        if risk_score.total_score >= 30 {
            risk_score.recommendations.push(
                "Monitorar tráfego de rede para padrões suspeitos".to_string()
            );
        }
    }
}

impl Default for RiskEngine {
    fn default() -> Self {
        Self::new()
    }
}
