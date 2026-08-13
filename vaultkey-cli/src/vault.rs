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
    pub category: Option<String>,
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
    let vault = Self::new(config)?;
    vault.init()?; // Ensure all tables exist
    Ok(vault)
}

    pub fn init(&self) -> Result<()> {
        // Create table with category column
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS secrets (
                name TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                password_encrypted BLOB NOT NULL,
                url TEXT,
                notes TEXT,
                category TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // Create password history table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS password_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                secret_name TEXT NOT NULL,
                password_encrypted BLOB NOT NULL,
                changed_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn add_secret_with_category(
        &mut self,
        name: &str,
        username: &str,
        password: &str,
        url: Option<&str>,
        notes: Option<&str>,
        category: Option<&str>,
        enc_key: &[u8; 32],
    ) -> Result<()> {
        let now = chrono::Utc::now();

        // Check if secret exists (for password history)
        if let Ok(old_secret) = self.get_secret(name, enc_key) {
            // Store old password in history
            let old_password_encrypted = crypto::encrypt(old_secret.password.as_bytes(), enc_key)?;
            self.conn.execute(
                "INSERT INTO password_history (secret_name, password_encrypted, changed_at)
                 VALUES (?1, ?2, ?3)",
                params![name, old_password_encrypted, now.to_rfc3339()],
            )?;
        }

        let password_encrypted = crypto::encrypt(password.as_bytes(), enc_key)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO secrets (name, username, password_encrypted, url, notes, category, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                name,
                username,
                password_encrypted,
                url,
                notes,
                category,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get_secret(&self, name: &str, enc_key: &[u8; 32]) -> Result<Secret> {
        let mut stmt = self.conn.prepare(
            "SELECT name, username, password_encrypted, url, notes, category, created_at, updated_at
             FROM secrets WHERE name = ?"
        )?;

        let row = stmt.query_row([name], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Vec<u8>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        })?;

        let (name, username, password_encrypted, url, notes, category, created_at, updated_at) = row;

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
            category,
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

    #[allow(dead_code)]
    pub fn export_encrypted(&self) -> Result<Vec<u8>> {
        let path = self.conn.path()
            .ok_or_else(|| anyhow::anyhow!("Database is in-memory"))?;
        std::fs::read(path).context("Failed to read vault file")
    }

    #[allow(dead_code)]
    pub fn import_encrypted(&mut self, data: &[u8]) -> Result<()> {
        let path = self.conn.path()
            .ok_or_else(|| anyhow::anyhow!("Database is in-memory"))?
            .to_string();
        std::fs::write(path, data).context("Failed to write vault file")
    }

    pub fn search_secrets(&self, query: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM secrets WHERE name LIKE ?1 OR username LIKE ?1 ORDER BY name"
        )?;

        let search_pattern = format!("%{}%", query);
        let names = stmt.query_map([&search_pattern], |row| row.get::<_, String>(0))?;

        let mut result = Vec::new();
        for name in names {
            result.push(name?);
        }

        Ok(result)
    }

    pub fn list_by_category(&self, category: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM secrets WHERE category = ? ORDER BY name"
        )?;
        let names = stmt.query_map([category], |row| row.get::<_, String>(0))?;

        let mut result = Vec::new();
        for name in names {
            result.push(name?);
        }

        Ok(result)
    }

    pub fn list_categories(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT DISTINCT category FROM secrets WHERE category IS NOT NULL ORDER BY category"
        )?;
        let categories = stmt.query_map([], |row| row.get::<_, String>(0))?;

        let mut result = Vec::new();
        for category in categories {
            result.push(category?);
        }

        Ok(result)
    }

    #[allow(dead_code)]
    pub fn get_password_history(&self, name: &str, enc_key: &[u8; 32]) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT password_encrypted, changed_at FROM password_history
             WHERE secret_name = ? ORDER BY changed_at DESC"
        )?;

        let rows = stmt.query_map([name], |row| {
            Ok((
                row.get::<_, Vec<u8>>(0)?,
                row.get::<_, String>(1)?,
            ))
        })?;

        let mut result = Vec::new();
        for row in rows {
            let (password_encrypted, changed_at) = row?;
            let password_bytes = crypto::decrypt(&password_encrypted, enc_key)?;
            let password = String::from_utf8(password_bytes)?;
            result.push((password, changed_at));
        }

        Ok(result)
    }

    pub fn mark_password_expired(&self, name: &str, days: i64) -> Result<()> {
        let expires_at = chrono::Utc::now() + chrono::Duration::days(days);

        // Try to add column if it doesn't exist
        self.conn.execute(
            "ALTER TABLE secrets ADD COLUMN expires_at TEXT",
            [],
        ).ok();

        self.conn.execute(
            "UPDATE secrets SET expires_at = ? WHERE name = ?",
            params![expires_at.to_rfc3339(), name],
        )?;

        Ok(())
    }

    pub fn check_expired_passwords(&self) -> Result<Vec<String>> {
        // Check if column exists
        let has_column = self.conn
            .prepare("SELECT expires_at FROM secrets LIMIT 1")
            .is_ok();

        if !has_column {
            return Ok(Vec::new());
        }

        let mut stmt = self.conn.prepare(
            "SELECT name, expires_at FROM secrets WHERE expires_at IS NOT NULL"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
            ))
        })?;

        let mut expired = Vec::new();
        let now = chrono::Utc::now();

        for row in rows {
            let (name, expires_at) = row?;
            if let Some(expires_str) = expires_at {
                if let Ok(expires) = chrono::DateTime::parse_from_rfc3339(&expires_str) {
                    if expires < now {
                        expired.push(name);
                    }
                }
            }
        }

        Ok(expired)
    }
}