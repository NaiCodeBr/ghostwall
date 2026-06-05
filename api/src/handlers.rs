use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::state::ApiState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[derive(Serialize)]
struct ScanResponse {
    success: bool,
    device_count: usize,
    message: String,
}

pub async fn start_scan(State(state): State<crate::ApiState>) -> impl IntoResponse {
    info!("Starting RF scan");
    
    let mut scanner = state.scanner.lock().await;
    match scanner.scan().await {
        Ok(devices) => {
            // Save devices to storage
            for device in &devices {
                let _ = state.storage.save_device(device).await;
            }
            
            Json(ScanResponse {
                success: true,
                device_count: devices.len(),
                message: format!("Discovered {} devices", devices.len()),
            })
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ScanResponse {
                success: false,
                device_count: 0,
                message: format!("Scan failed: {}", e),
            }))
        }
    }
}

#[derive(Serialize)]
struct DevicesResponse {
    devices: Vec<ghostwall_core::Device>,
}

pub async fn get_devices(State(state): State<crate::ApiState>) -> impl IntoResponse {
    match state.storage.get_devices().await {
        Ok(devices) => Json(DevicesResponse { devices }),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get devices: {}", e)).into_response()
        }
    }
}

pub async fn get_device(
    State(state): State<crate::ApiState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    // Implementation would get specific device by ID
    (StatusCode::NOT_FOUND, "Device not found").into_response()
}

#[derive(Deserialize)]
struct RiskEvaluationRequest {
    device_id: String,
}

#[derive(Serialize)]
struct RiskEvaluationResponse {
    device_id: String,
    risk_score: ghostwall_core::RiskScore,
}

pub async fn evaluate_risk(
    State(state): State<crate::ApiState>,
    Json(request): Json<RiskEvaluationRequest>,
) -> impl IntoResponse {
    // Get device from storage
    let devices = state.storage.get_devices().await.unwrap_or_default();
    
    if let Some(device) = devices.iter().find(|d| d.id.to_string() == request.device_id) {
        let risk_score = state.risk_engine.evaluate_device(device, None);
        
        // Save risk score
        let _ = state.storage.save_risk_score(&request.device_id, &risk_score).await;
        
        Json(RiskEvaluationResponse {
            device_id: request.device_id,
            risk_score,
        })
    } else {
        (StatusCode::NOT_FOUND, "Device not found").into_response()
    }
}

#[derive(Serialize)]
struct ExposureResponse {
    exposure_score: ghostwall_core::ExposureScore,
}

pub async fn get_exposure(State(state): State<crate::ApiState>) -> impl IntoResponse {
    let devices = state.storage.get_devices().await.unwrap_or_default();
    let exposure_scanner = ghostwall_storage::exposure::ExposureScanner::new();
    let exposure_score = exposure_scanner.calculate_exposure(&devices);
    
    // Save exposure score
    let _ = state.storage.save_exposure_score(&exposure_score).await;
    
    Json(ExposureResponse { exposure_score })
}

#[derive(Deserialize)]
struct ReportRequest {
    format: String,
}

#[derive(Serialize)]
struct ReportResponse {
    report_id: String,
    content: String,
}

pub async fn generate_report(
    State(state): State<crate::ApiState>,
    Json(request): Json<ReportRequest>,
) -> impl IntoResponse {
    let devices = state.storage.get_devices().await.unwrap_or_default();
    let exposure_scanner = ghostwall_storage::exposure::ExposureScanner::new();
    let exposure_score = exposure_scanner.calculate_exposure(&devices);
    
    // Generate risk scores for all devices
    let mut risk_scores = Vec::new();
    for device in &devices {
        let risk_score = state.risk_engine.evaluate_device(device, None);
        risk_scores.push(risk_score);
    }
    
    let report = state.report_engine.generate_report(devices, risk_scores, exposure_score);
    
    let content = match request.format.as_str() {
        "html" => state.report_engine.export_html(&report).unwrap_or_default(),
        "json" => state.report_engine.export_json(&report).unwrap_or_default(),
        _ => state.report_engine.export_json(&report).unwrap_or_default(),
    };
    
    Json(ReportResponse {
        report_id: report.id,
        content,
    })
}

#[derive(Serialize)]
struct ThreatIntelResponse {
    signatures: Vec<ghostwall_threat_intel::ThreatSignature>,
    version: String,
    last_updated: String,
}

pub async fn get_threat_intel(State(state): State<crate::ApiState>) -> impl IntoResponse {
    let threat_db = state.threat_db.lock().await;
    let signatures = threat_db.signatures.clone();
    let version = threat_db.version().to_string();
    let last_updated = threat_db.last_updated().to_rfc3339();
    
    Json(ThreatIntelResponse {
        signatures,
        version,
        last_updated,
    })
}
