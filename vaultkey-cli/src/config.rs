use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub server_url: String,
    pub vault_path: PathBuf,
    pub token: Option<String>,
    #[serde(default = "default_clipboard_timeout")]
    pub clipboard_timeout: u64,
}

fn default_clipboard_timeout() -> u64 {
    30
}

impl Config {
    pub fn new(username: String, server_url: String) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("vaultkey");

        std::fs::create_dir_all(&config_dir)?;

        let vault_path = config_dir.join("vault.db");

        Ok(Config {
            username,
            server_url,
            vault_path,
            token: None,
            clipboard_timeout: default_clipboard_timeout(),
        })
    }

    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("vaultkey");

        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            anyhow::bail!("Configuration not found. Run 'vaultkey init' first.");
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("vaultkey");

        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }
}