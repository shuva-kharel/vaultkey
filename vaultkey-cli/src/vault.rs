use anyhow::{Result, Context};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::crypto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct Vault {
    conn: Connection,
}

impl Vault {
    pub fn new(config: &Config) -> Result<Self> {
        let conn = Connection::open(&config.vault_path)?;
        Ok(Vault { conn })
    }

    pub fn open(config: &Config) -> Result<Self> {
        if !config.vault_path.exists() {
            anyhow::bail!("Vault not initialized. Run 'vaultkey init' first.");
        }
        Self::new(config)
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                name TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                password_encrypted BLOB NOT NULL,
                url TEXT,
                notes TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_secret(
        &mut self,
        name: &str,
        username: &str,
        password: &str,
        url: Option<&str>,
        notes: Option<&str>,
        enc_key: &[u8; 32],
    ) -> Result<()> {
        let now = chrono::Utc::now();

        // Encrypt the password
        let password_encrypted = crypto::encrypt(password.as_bytes(), enc_key)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO secrets (name, username, password_encrypted, url, notes, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                name,
                username,
                password_encrypted,
                url,
                notes,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get_secret(&self, name: &str, enc_key: &[u8; 32]) -> Result<Secret> {
        let mut stmt = self.conn.prepare(
            "SELECT name, username, password_encrypted, url, notes, created_at, updated_at
             FROM secrets WHERE name = ?"
        )?;

        // First, get the raw data
        let row = stmt.query_row([name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })?;

        // Now decrypt outside the closure where we can use anyhow errors
        let (name, username, password_encrypted, url, notes, created_at, updated_at) = row;

        let password_bytes = crypto::decrypt(&password_encrypted, enc_key)?;
        let password = String::from_utf8(password_bytes)?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)?
            .with_timezone(&chrono::Utc);
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at)?
            .with_timezone(&chrono::Utc);

        Ok(Secret {
            name,
            username,
            password,
            url,
            notes,
            created_at,
            updated_at,
        })
    }

    pub fn list_secrets(&self, _enc_key: &[u8; 32]) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM secrets ORDER BY name")?;
        let names = stmt.query_map([], |row| row.get::<_, String>(0))?;

        let mut result = Vec::new();
        for name in names {
            result.push(name?);
        }

        Ok(result)
    }

    pub fn delete_secret(&mut self, name: &str, _enc_key: &[u8; 32]) -> Result<()> {
        self.conn.execute("DELETE FROM secrets WHERE name = ?", [name])?;
        Ok(())
    }

    /// Export all encrypted data for sync
    #[allow(dead_code)]
    pub fn export_encrypted(&self) -> Result<Vec<u8>> {
        // For now, just export the raw database file
        let path = self.conn.path()
            .ok_or_else(|| anyhow::anyhow!("Database is in-memory"))?;
        std::fs::read(path).context("Failed to read vault file")
    }

    /// Import encrypted data from sync
    #[allow(dead_code)]
    pub fn import_encrypted(&mut self, data: &[u8]) -> Result<()> {
        // For now, just write the raw database file
        let path = self.conn.path()
            .ok_or_else(|| anyhow::anyhow!("Database is in-memory"))?
            .to_string();
        std::fs::write(path, data).context("Failed to write vault file")
    }
}