use crate::azure::cli::run_az;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureAccount {
    pub name: String,
    pub id: String,
    pub tenant_id: String,
    pub user: AccountUser,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountUser {
    pub name: String,
    pub r#type: String,
}

pub fn get_account() -> Result<AzureAccount, Box<dyn std::error::Error>> {
    let output = run_az(&["account", "show", "--output", "json"])?;

    let account: AzureAccount = serde_json::from_str(&output)?;

    Ok(account)
}

pub fn get_account_list() -> Result<Vec<AzureAccount>, Box<dyn std::error::Error>> {
    let output = run_az(&["account", "list", "--output", "json"])?;

    let accounts: Vec<AzureAccount> = serde_json::from_str(&output)?;

    Ok(accounts)
}

pub fn set_account(subscription_id: String) -> Result<(), Box<dyn std::error::Error>> {
    run_az(&["account", "set", "--subscription", subscription_id.as_str()])?;
    Ok(())
}