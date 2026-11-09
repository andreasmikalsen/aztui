use crate::azure::cli::run_az;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessToken {
    pub access_token: String,
    pub expires_on: String,
    #[serde(rename = "expires_on")]
    pub expires_on_timestamp: Option<i64>,
    pub token_type: String,
    pub subscription: String,
    pub tenant: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenTarget {
    Database,
    Redis,
}

impl TokenTarget {
    pub const ALL: [TokenTarget; 2] = [TokenTarget::Database, TokenTarget::Redis];

    pub fn title(&self) -> &str {
        match self {
            TokenTarget::Database => "Database",
            TokenTarget::Redis => "Redis",
        }
    }

    pub fn scope(&self) -> &str {
        match self {
            TokenTarget::Database => "https://ossrdbms-aad.database.windows.net/.default",
            TokenTarget::Redis => "https://redis.azure.com/.default",
        }
    }
}

pub fn get_access_token(
    target: TokenTarget,
    subscription_id: &str,
) -> Result<AccessToken, Box<dyn std::error::Error>> {
    let output = run_az(&[
        "account",
        "get-access-token",
        "--scope",
        target.scope(),
        "--subscription",
        subscription_id,
        "--output",
        "json",
    ])?;

    let token: AccessToken = serde_json::from_str(&output)?;

    Ok(token)
}
