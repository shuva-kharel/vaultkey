use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::config::Config;
use crate::vault::Vault;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub version: String,
    pub exported_at: String,
    pub secrets: Vec<ExportSecret>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportSecret {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category: Option<String>,
}

pub fn export_vault(config: &Config, enc_key: &[u8; 32], output_path: &Path) -> Result<()> {
    let vault = Vault::open(config)?;
    let secret_names = vault.list_secrets(enc_key)?;

    let mut secrets = Vec::new();
    for name in secret_names {
        let secret = vault.get_secret(&name, enc_key)?;
        secrets.push(ExportSecret {
            name: secret.name,
            username: secret.username,
            password: secret.password,
            url: secret.url,
            notes: secret.notes,
            category: secret.category,
        });
    }

    let export_data = ExportData {
        version: "1.0".to_string(),
        exported_at: chrono::Utc::now().to_rfc3339(),
        secrets,
    };

    let json = serde_json::to_string_pretty(&export_data)?;
    std::fs::write(output_path, json)?;

    println!("[✓] Exported {} secrets to {}", export_data.secrets.len(), output_path.display());
    println!("[!] Warning: This file contains plaintext passwords. Keep it secure!");

    Ok(())
}

pub fn import_vault(config: &Config, enc_key: &[u8; 32], input_path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(input_path)?;
    let import_data: ExportData = serde_json::from_str(&content)?;

    let mut vault = Vault::open(config)?;
    vault.init()?; // Ensure all tables exist

    for secret in import_data.secrets {
        vault.add_secret_with_category(
            &secret.name,
            &secret.username,
            &secret.password,
            secret.url.as_deref(),
            secret.notes.as_deref(),
            secret.category.as_deref(),
            enc_key,
        )?;
        println!("  [✓] Imported: {}", secret.name);
    }

    println!("[✓] Import complete");
    Ok(())
}