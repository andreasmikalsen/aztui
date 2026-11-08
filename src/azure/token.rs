use crate::azure::cli::run_az;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    pub access_token: String,
    pub expires_on: String,
    pub expires_on_timestamp: Option<i64>,
    pub token_type: String,
    pub subscription: String,
    pub tenant: String,
}

pub fn get_vault_token() -> Result<AccessToken, Box<dyn std::error::Error>> {
    let output = run_az(&[
        "account",
        "get-access-token",
        "--resource",
        "https://vault.azure.net",
        "--output",
        "json",
    ])?;

    let token: AccessToken = serde_json::from_str(&output)?;

    Ok(token)
}
