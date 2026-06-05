use thiserror::Error;

pub type Result<T> = std::result::Result<T, GhostwallError>;

#[derive(Error, Debug)]
pub enum GhostwallError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Scan error: {0}")]
    ScanError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
