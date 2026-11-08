use crate::azure::cli::run_az;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyVault {
    pub name: String,
    pub id: String,
    pub location: String,
}

#[derive(Debug, Deserialize)]
pub struct Secret {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SecretValue {
    pub value: String,
}

pub fn list_vaults() -> Result<Vec<KeyVault>, Box<dyn std::error::Error>> {
    let output = run_az(&["keyvault", "list", "--output", "json"])?;

    let vaults: Vec<KeyVault> = serde_json::from_str(&output)?;

    Ok(vaults)
}

pub fn list_secrets(vault_name: &str) -> Result<Vec<Secret>, Box<dyn std::error::Error>> {
    let output = run_az(&[
        "keyvault",
        "secret",
        "list",
        "--vault-name",
        vault_name,
        "--output",
        "json",
    ])?;

    let secrets: Vec<Secret> = serde_json::from_str(&output)?;

    Ok(secrets)
}

pub fn get_secret(
    vault_name: &str,
    secret_name: &str,
) -> Result<SecretValue, Box<dyn std::error::Error>> {
    let output = run_az(&[
        "keyvault",
        "secret",
        "show",
        "--vault-name",
        vault_name,
        "--name",
        secret_name,
        "--output",
        "json",
    ])?;

    let secret: SecretValue = serde_json::from_str(&output)?;

    Ok(secret)
}
