use ghostwall_core::{Device, RiskScore, ExposureScore, Result};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use tracing::info;

/// Report Engine - Module 6
/// Generates reports in PDF, HTML, and JSON formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub title: String,
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub inventory: Vec<Device>,
    pub risk_scores: Vec<RiskScore>,
    pub exposure_score: ExposureScore,
    pub suspicious_devices: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportFormat {
    Json,
    Html,
    Pdf,
}

pub struct ReportEngine;

impl ReportEngine {
    pub fn new() -> Self {
        Self
    }

    /// Generate a comprehensive report
    pub fn generate_report(
        &self,
        inventory: Vec<Device>,
        risk_scores: Vec<RiskScore>,
        exposure_score: ExposureScore,
    ) -> Report {
        info!("Generating report");
        
        let suspicious_devices = self.identify_suspicious_devices(&inventory, &risk_scores);
        let recommendations = self.generate_recommendations(&risk_scores, &exposure_score);
        
        Report {
            id: uuid::Uuid::new_v4().to_string(),
            title: "GHOSTWALL RF Privacy Report".to_string(),
            generated_at: Utc::now(),
            inventory,
            risk_scores,
            exposure_score,
            suspicious_devices,
            recommendations,
        }
    }

    fn identify_suspicious_devices(&self, inventory: &[Device], risk_scores: &[RiskScore]) -> Vec<String> {
        let mut suspicious = Vec::new();
        
        for (device, risk) in inventory.iter().zip(risk_scores.iter()) {
            if risk.total_score >= 40 {
                suspicious.push(format!(
                    "{} ({}) - Score: {}",
                    device.mac_address,
                    device.vendor.as_deref().unwrap_or("Unknown"),
                    risk.total_score
                ));
            }
        }
        
        suspicious
    }

    fn generate_recommendations(&self, risk_scores: &[RiskScore], exposure_score: &ExposureScore) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        // High-level exposure recommendations
        if exposure_score.score > 70 {
            recommendations.push(
                "Alta exposição RF detectada. Considere reduzir o número de APs ou dispositivos.".to_string()
            );
        }
        
        // Risk-based recommendations
        for risk in risk_scores {
            if risk.total_score >= 50 {
                recommendations.extend(risk.recommendations.clone());
            }
        }
        
        // General recommendations
        recommendations.push(
            "Realize varreduras regulares para monitorar mudanças na infraestrutura RF.".to_string()
        );
        
        recommendations.push(
            "Mantenha o firmware dos dispositivos atualizado.".to_string()
        );
        
        recommendations
    }

    /// Export report to JSON
    pub fn export_json(&self, report: &Report) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    /// Export report to HTML
    pub fn export_html(&self, report: &Report) -> Result<String> {
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        h1 {{ color: #333; }}
        h2 {{ color: #666; margin-top: 30px; }}
        table {{ border-collapse: collapse; width: 100%; margin-top: 20px; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}
        th {{ background-color: #f2f2f2; }}
        .high-risk {{ background-color: #ffcccc; }}
        .medium-risk {{ background-color: #ffffcc; }}
        .low-risk {{ background-color: #ccffcc; }}
        .score {{ font-weight: bold; }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <p><strong>Gerado em:</strong> {}</p>
    
    <h2>Score de Exposição RF</h2>
    <p class="score">{} / 100</p>
    <ul>
        <li>Cobertura Wi-Fi: {:.1}%</li>
        <li>APs detectados: {}</li>
        <li>Dispositivos: {}</li>
        <li>Densidade RF: {:.1}</li>
    </ul>
    
    <h2>Inventário de Dispositivos</h2>
    <table>
        <tr>
            <th>MAC Address</th>
            <th>Vendor</th>
            <th>SSID</th>
            <th>RSSI</th>
            <th>Canal</th>
            <th>Tipo</th>
        </tr>
{}
    </table>
    
    <h2>Dispositivos Suspeitos</h2>
    <ul>
{}
    </ul>
    
    <h2>Recomendações</h2>
    <ul>
{}
    </ul>
</body>
</html>"#,
            report.title,
            report.title,
            report.generated_at.format("%Y-%m-%d %H:%M:%S UTC"),
            report.exposure_score.score,
            report.exposure_score.metrics.wifi_coverage * 100.0,
            report.exposure_score.metrics.ap_count,
            report.exposure_score.metrics.device_count,
            report.exposure_score.metrics.rf_density,
            report.inventory.iter()
                .map(|d| format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{:?}</td></tr>",
                    d.mac_address,
                    d.vendor.as_deref().unwrap_or("Unknown"),
                    d.ssid.as_deref().unwrap_or("N/A"),
                    d.rssi,
                    d.channel,
                    d.device_type
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            report.suspicious_devices.iter()
                .map(|d| format!("<li>{}</li>", d))
                .collect::<Vec<_>>()
                .join("\n"),
            report.recommendations.iter()
                .map(|r| format!("<li>{}</li>", r))
                .collect::<Vec<_>>()
                .join("\n")
        );
        
        Ok(html)
    }

    /// Export report to PDF (placeholder - would use a PDF library)
    pub fn export_pdf(&self, report: &Report) -> Result<Vec<u8>> {
        // PDF generation would require a library like `printpdf` or `lopdf`
        // For now, return the HTML as a placeholder
        let html = self.export_html(report)?;
        Ok(html.into_bytes())
    }
}

impl Default for ReportEngine {
    fn default() -> Self {
        Self::new()
    }
}
