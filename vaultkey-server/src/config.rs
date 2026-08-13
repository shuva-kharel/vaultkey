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
        // Try to load from environment variables first (for Docker)
        if let Ok(config) = Self::from_env() {
            return Ok(config);
        }

        // Fallback to file
        let config_content = std::fs::read_to_string("server.toml")?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }

    fn from_env() -> Result<Self> {
        let listen_addr = std::env::var("VAULTKEY_LISTEN_ADDR")
            .unwrap_or_else(|_| "127.0.0.1".to_string());
        let listen_port = std::env::var("VAULTKEY_LISTEN_PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()?;
        let database_url = std::env::var("VAULTKEY_DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://data/vaultkey.db".to_string());
        let rp_id = std::env::var("VAULTKEY_RP_ID")
            .unwrap_or_else(|_| "localhost".to_string());
        let rp_origin = std::env::var("VAULTKEY_RP_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
        let storage_root = std::env::var("VAULTKEY_STORAGE_ROOT")
            .unwrap_or_else(|_| "./storage".to_string());
        let jwt_secret = std::env::var("VAULTKEY_JWT_SECRET")
            .unwrap_or_else(|_| "change-me".to_string());
        let jwt_expiration_hours = std::env::var("VAULTKEY_JWT_EXPIRATION_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()?;

        Ok(Config {
            server: ServerConfig { listen_addr, listen_port },
            database: DatabaseConfig { url: database_url },
            webauthn: WebauthnConfig { rp_id, rp_origin },
            storage: StorageConfig { root: PathBuf::from(storage_root) },
            jwt: JwtConfig {
                secret: jwt_secret,
                expiration_hours: jwt_expiration_hours,
            },
        })
    }
}