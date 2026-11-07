use serde::Deserialize;
use crate::azure::cli::run_az;

#[derive(Debug, Deserialize)]
pub struct AzureAccount {
    pub name: String,
    pub id: String,
    pub tenant_id: String,
    pub user: AccountUser,
}

#[derive(Debug, Deserialize)]
pub struct AccountUser {
    pub name: String,
    pub r#type: String,
}

pub fn get_account() -> Result<AzureAccount, Box<dyn std::error::Error>> {
    let output = run_az(&[
        "account",
        "show",
        "--output",
        "json",
    ])?;

    let account: AzureAccount = serde_json::from_str(&output)?;

    Ok(account)
}