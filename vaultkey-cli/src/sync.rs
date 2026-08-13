use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::vault::Vault;
use crate::hardware_key::HardwareKey;

#[derive(Debug, Serialize, Deserialize)]
struct ServerSecret {
    name: String,
    username: String,
    password: String,
    url: Option<String>,
    notes: Option<String>,
    category: Option<String>,
}

pub async fn sync(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();
    let hw_key = HardwareKey::new()?;
    let enc_key = hw_key.derive_key()?;
    let vault = Vault::open(config)?;

    // Step 1: Get local secrets
    let local_secret_names = vault.list_secrets(&enc_key)?;

    // Step 2: Upload local secrets to server
    println!("[*] Uploading local secrets to server...");
    for name in &local_secret_names {
        let secret = vault.get_secret(name, &enc_key)?;

        let server_secret = ServerSecret {
            name: secret.name.clone(),
            username: secret.username.clone(),
            password: secret.password.clone(),
            url: secret.url.clone(),
            notes: secret.notes.clone(),
            category: secret.category.clone(),
        };

        let resp = client
            .post(format!("{}/vault/secrets", config.server_url))
            .bearer_auth(token)
            .json(&server_secret)
            .send()
            .await?;

        if resp.status().is_success() {
            println!("  [✓] Uploaded: {}", name);
        } else {
            println!("  [!] Failed to upload: {} ({})", name, resp.status());
        }
    }

    // Step 3: Download server secrets
    println!("[*] Downloading secrets from server...");
    let resp = client
        .get(format!("{}/vault/secrets", config.server_url))
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status().is_success() {
        let server_secrets: Vec<ServerSecret> = resp.json().await?;

        let mut vault = Vault::open(config)?;
        for secret in server_secrets {
            vault.add_secret_with_category(
                &secret.name,
                &secret.username,
                &secret.password,
                secret.url.as_deref(),
                secret.notes.as_deref(),
                secret.category.as_deref(),
                &enc_key,
            )?;
            println!("  [✓] Downloaded: {}", secret.name);
        }
    } else {
        println!("  [!] Failed to download: {}", resp.status());
    }

    println!("[✓] Sync complete");
    Ok(())
}

pub async fn upload(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();
    let hw_key = HardwareKey::new()?;
    let enc_key = hw_key.derive_key()?;
    let vault = Vault::open(config)?;

    println!("[*] Uploading local secrets to server...");
    let local_secret_names = vault.list_secrets(&enc_key)?;

    for name in &local_secret_names {
        let secret = vault.get_secret(name, &enc_key)?;

        let server_secret = ServerSecret {
            name: secret.name.clone(),
            username: secret.username.clone(),
            password: secret.password.clone(),
            url: secret.url.clone(),
            notes: secret.notes.clone(),
            category: secret.category.clone(),
        };

        let resp = client
            .post(format!("{}/vault/secrets", config.server_url))
            .bearer_auth(token)
            .json(&server_secret)
            .send()
            .await?;

        if resp.status().is_success() {
            println!("  [✓] Uploaded: {}", name);
        } else {
            println!("  [!] Failed to upload: {} ({})", name, resp.status());
        }
    }

    println!("[✓] Upload complete");
    Ok(())
}

pub async fn download(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();
    let hw_key = HardwareKey::new()?;
    let enc_key = hw_key.derive_key()?;

    println!("[*] Downloading secrets from server...");
    let resp = client
        .get(format!("{}/vault/secrets", config.server_url))
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status().is_success() {
        let server_secrets: Vec<ServerSecret> = resp.json().await?;

        let mut vault = Vault::open(config)?;
        for secret in server_secrets {
            vault.add_secret_with_category(
                &secret.name,
                &secret.username,
                &secret.password,
                secret.url.as_deref(),
                secret.notes.as_deref(),
                secret.category.as_deref(),
                &enc_key,
            )?;
            println!("  [✓] Downloaded: {}", secret.name);
        }

        println!("[✓] Download complete");
    } else {
        println!("[!] No secrets found on server or download failed: {}", resp.status());
    }

    Ok(())
}