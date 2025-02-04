use crate::error::AmdDriverError;
use serde_json::Value;

pub async fn fetch_manifest() -> Result<Value, AmdDriverError> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.amd-drivers.net/v1/manifest")
        .send()
        .await?;
    Ok(response.json().await?)
}