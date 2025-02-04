use thiserror::Error;

#[derive(Error, Debug)]
pub enum AmdDriverError {
    #[error("Hardware detection failed")]
    DetectionFailure,
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Driver installation failed")]
    InstallationFailure,
    #[error("Invalid manifest format")]
    ManifestParseError,
}