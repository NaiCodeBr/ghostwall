use ghostwall_core::{Device, Fingerprint, RiskScore, ExposureScore, ExposureMetrics, Result, GhostwallError};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use chrono::Utc;
use tracing::{info, error};
use std::path::Path;

pub mod exposure;
pub mod schema;

pub use exposure::ExposureScanner;
pub use schema::Database;

/// Storage layer with SQLite support
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    /// Create new storage instance with SQLite database
    pub async fn new(database_path: &Path) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(database_path)
            .create_if_missing(true);
        
        let pool = SqlitePool::connect_with(options).await?;
        
        // Run migrations
        Self::run_migrations(&pool).await?;
        
        Ok(Self { pool })
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS devices (
                id TEXT PRIMARY KEY,
                mac_address TEXT NOT NULL,
                vendor TEXT,
                ssid TEXT,
                bssid TEXT,
                rssi INTEGER NOT NULL,
                channel INTEGER NOT NULL,
                security TEXT NOT NULL,
                device_type TEXT NOT NULL,
                first_seen TEXT NOT NULL,
                last_seen TEXT NOT NULL,
                is_active INTEGER NOT NULL,
                estimated_uptime_seconds INTEGER
            )
            "#
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fingerprints (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                signature TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (device_id) REFERENCES devices(id)
            )
            "#
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS risk_scores (
                id TEXT PRIMARY KEY,
                device_id TEXT NOT NULL,
                total_score INTEGER NOT NULL,
                level TEXT NOT NULL,
                applied_rules TEXT NOT NULL,
                recommendations TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (device_id) REFERENCES devices(id)
            )
            "#
        )
        .execute(pool)
        .await?;
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS exposure_scores (
                id TEXT PRIMARY KEY,
                score INTEGER NOT NULL,
                wifi_coverage REAL NOT NULL,
                ap_count INTEGER NOT NULL,
                device_count INTEGER NOT NULL,
                rf_density REAL NOT NULL,
                spatial_distribution REAL NOT NULL,
                factors TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Save or update a device
    pub async fn save_device(&self, device: &Device) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO devices 
            (id, mac_address, vendor, ssid, bssid, rssi, channel, security, device_type, first_seen, last_seen, is_active, estimated_uptime_seconds)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(device.id.to_string())
        .bind(&device.mac_address)
        .bind(&device.vendor)
        .bind(&device.ssid)
        .bind(&device.bssid)
        .bind(device.rssi)
        .bind(device.channel)
        .bind(serde_json::to_string(&device.security)?)
        .bind(serde_json::to_string(&device.device_type)?)
        .bind(device.first_seen.to_rfc3339())
        .bind(device.last_seen.to_rfc3339())
        .bind(device.is_active as i32)
        .bind(device.estimated_uptime_seconds.map(|u| u as i32))
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Get all devices
    pub async fn get_devices(&self) -> Result<Vec<Device>> {
        let rows = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<String>, i32, u32, String, String, String, String, i32, Option<i32>)>(
            "SELECT id, mac_address, vendor, ssid, bssid, rssi, channel, security, device_type, first_seen, last_seen, is_active, estimated_uptime_seconds FROM devices"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut devices = Vec::new();
        for row in rows {
            let device = Device {
                id: uuid::Uuid::parse_str(&row.0)?,
                mac_address: row.1,
                vendor: row.2,
                ssid: row.3,
                bssid: row.4,
                rssi: row.5,
                channel: row.6,
                security: serde_json::from_str(&row.7)?,
                device_type: serde_json::from_str(&row.8)?,
                first_seen: chrono::DateTime::parse_from_rfc3339(&row.9)?.with_timezone(&Utc),
                last_seen: chrono::DateTime::parse_from_rfc3339(&row.10)?.with_timezone(&Utc),
                is_active: row.11 != 0,
                estimated_uptime_seconds: row.12.map(|u| u as u64),
            };
            devices.push(device);
        }
        
        Ok(devices)
    }

    /// Save a fingerprint
    pub async fn save_fingerprint(&self, fingerprint: &Fingerprint) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO fingerprints (id, device_id, signature, data, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&fingerprint.id)
        .bind(&fingerprint.device_id)
        .bind(&fingerprint.signature)
        .bind(serde_json::to_string(&fingerprint.data)?)
        .bind(fingerprint.created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Save a risk score
    pub async fn save_risk_score(&self, device_id: &str, risk_score: &RiskScore) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO risk_scores (id, device_id, total_score, level, applied_rules, recommendations, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(device_id)
        .bind(risk_score.total_score)
        .bind(risk_score.level.as_str())
        .bind(serde_json::to_string(&risk_score.applied_rules)?)
        .bind(serde_json::to_string(&risk_score.recommendations)?)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    /// Save an exposure score
    pub async fn save_exposure_score(&self, exposure_score: &ExposureScore) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO exposure_scores (id, score, wifi_coverage, ap_count, device_count, rf_density, spatial_distribution, factors, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(exposure_score.score)
        .bind(exposure_score.metrics.wifi_coverage)
        .bind(exposure_score.metrics.ap_count)
        .bind(exposure_score.metrics.device_count)
        .bind(exposure_score.metrics.rf_density)
        .bind(exposure_score.metrics.spatial_distribution)
        .bind(serde_json::to_string(&exposure_score.factors)?)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
