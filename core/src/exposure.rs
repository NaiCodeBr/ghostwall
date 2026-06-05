use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics {
    pub wifi_coverage: f64,
    pub ap_count: u32,
    pub device_count: u32,
    pub rf_density: f64,
    pub spatial_distribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExposureScore {
    pub score: u32,
    pub metrics: ExposureMetrics,
    pub factors: Vec<String>,
}

impl ExposureScore {
    pub fn new(metrics: ExposureMetrics) -> Self {
        let score = Self::calculate_score(&metrics);
        let factors = Self::identify_factors(&metrics);
        
        Self {
            score,
            metrics,
            factors,
        }
    }

    fn calculate_score(metrics: &ExposureMetrics) -> u32 {
        let mut score = 0u32;
        
        // WiFi coverage contribution (0-25)
        score += (metrics.wifi_coverage * 25.0) as u32;
        
        // AP count contribution (0-20)
        score += (metrics.ap_count.min(10) * 2) as u32;
        
        // Device count contribution (0-20)
        score += (metrics.device_count.min(20)) as u32;
        
        // RF density contribution (0-20)
        score += (metrics.rf_density * 20.0) as u32;
        
        // Spatial distribution contribution (0-15)
        score += (metrics.spatial_distribution * 15.0) as u32;
        
        score.min(100)
    }

    fn identify_factors(metrics: &ExposureMetrics) -> Vec<String> {
        let mut factors = Vec::new();
        
        if metrics.wifi_coverage > 0.8 {
            factors.push("Alta cobertura Wi-Fi detectada".to_string());
        }
        
        if metrics.ap_count > 5 {
            factors.push(format!("Múltiplos APs detectados: {}", metrics.ap_count));
        }
        
        if metrics.device_count > 15 {
            factors.push(format!("Alta densidade de dispositivos: {}", metrics.device_count));
        }
        
        if metrics.rf_density > 0.7 {
            factors.push("Alta densidade RF detectada".to_string());
        }
        
        if factors.is_empty() {
            factors.push("Nenhum fator de exposição significativo".to_string());
        }
        
        factors
    }
}
