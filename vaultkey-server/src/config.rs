use serde::Deserialize;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub webauthn: WebauthnConfig,
    pub storage: StorageConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: String,
    pub listen_port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WebauthnConfig {
    pub rp_id: String,
    pub rp_origin: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub root: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expiration_hours: i64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_content = std::fs::read_to_string("server.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}