use anyhow::{Result, Context};
use crate::config::Config;

pub async fn sync(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();

    // Step 1: Read local vault
    println!("[*] Reading local vault...");
    let vault_data = std::fs::read(&config.vault_path)
        .context("Failed to read vault file")?;

    // Step 2: Upload to server
    println!("[*] Uploading vault to server...");
    let resp = client
        .put(format!("{}/vault", config.server_url))
        .bearer_auth(token)
        .json(&serde_json::json!({
            "data": vault_data,
            "version": 1,
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Upload failed: {}", resp.status());
    }

    println!("[✓] Vault uploaded successfully");

    // Step 3: Download latest version from server
    println!("[*] Checking for updates...");
    let resp = client
        .get(format!("{}/vault", config.server_url))
        .bearer_auth(token)
        .send()
        .await?;

    if resp.status().is_success() {
        let data = resp.bytes().await?;
        std::fs::write(&config.vault_path, data)
            .context("Failed to write vault file")?;
        println!("[✓] Vault synced with server");
    } else if resp.status() == 404 {
        println!("[!] No vault found on server");
    } else {
        anyhow::bail!("Download failed: {}", resp.status());
    }

    Ok(())
}

pub async fn download(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();

    println!("[*] Downloading vault from server...");
    let resp = client
        .get(format!("{}/vault", config.server_url))
        .bearer_auth(token)
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Download failed: {}", resp.status());
    }

    let data = resp.bytes().await?;
    std::fs::write(&config.vault_path, data)
        .context("Failed to write vault file")?;

    println!("[✓] Vault downloaded successfully");
    Ok(())
}

pub async fn upload(config: &Config) -> Result<()> {
    let token = config.token.as_ref()
        .context("Not logged in. Please login first.")?;

    let client = reqwest::Client::new();

    println!("[*] Reading local vault...");
    let vault_data = std::fs::read(&config.vault_path)
        .context("Failed to read vault file")?;

    println!("[*] Uploading vault to server...");
    let resp = client
        .put(format!("{}/vault", config.server_url))
        .bearer_auth(token)
        .json(&serde_json::json!({
            "data": vault_data,
            "version": 1,
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        anyhow::bail!("Upload failed: {}", resp.status());
    }

    println!("[✓] Vault uploaded successfully");
    Ok(())
}