use crate::error::AmdDriverError;
use std::path::PathBuf;

pub struct DriverManager {
    install_path: PathBuf,
}

impl DriverManager {
    pub fn new() -> Result<Self, AmdDriverError> {
        let mut path = dirs::data_dir().ok_or(AmdDriverError::InstallationFailure)?;
        path.push("AMD_Drivers");
        std::fs::create_dir_all(&path)?;
        Ok(Self { install_path: path })
    }

    pub fn check_installed_version(&self, component: &str) -> Option<String> {
        let mut path = self.install_path.clone();
        path.push(format!("{}.version", component));
        std::fs::read_to_string(path).ok()
    }

    pub async fn install_driver(&self, url: &str) -> Result<(), AmdDriverError> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        let content = response.bytes().await?;
        let mut dest = self.install_path.clone();
        dest.push("driver_package.zip");
        tokio::fs::write(&dest, &content).await?;
        Ok(())
    }
}