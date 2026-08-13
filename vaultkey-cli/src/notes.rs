use anyhow::Result;
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::crypto;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureNote {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct NotesManager {
    conn: Connection,
}

impl NotesManager {
    pub fn new(config: &Config) -> Result<Self> {
        let conn = Connection::open(&config.vault_path)?;
        Ok(NotesManager { conn })
    }

    pub fn init(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                title TEXT PRIMARY KEY,
                content_encrypted BLOB NOT NULL,
                category TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn add_note(
        &mut self,
        title: &str,
        content: &str,
        category: Option<&str>,
        enc_key: &[u8; 32],
    ) -> Result<()> {
        let now = chrono::Utc::now();
        let content_encrypted = crypto::encrypt(content.as_bytes(), enc_key)?;

        self.conn.execute(
            "INSERT OR REPLACE INTO notes (title, content_encrypted, category, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                title,
                content_encrypted,
                category,
                now.to_rfc3339(),
                now.to_rfc3339(),
            ],
        )?;

        Ok(())
    }

    pub fn get_note(&self, title: &str, enc_key: &[u8; 32]) -> Result<SecureNote> {
        let mut stmt = self.conn.prepare(
            "SELECT title, content_encrypted, category, created_at, updated_at
             FROM notes WHERE title = ?"
        )?;

        let row = stmt.query_row([title], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })?;

        let (title, content_encrypted, category, created_at, updated_at) = row;

        let content_bytes = crypto::decrypt(&content_encrypted, enc_key)?;
        let content = String::from_utf8(content_bytes)?;

        let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)?
            .with_timezone(&chrono::Utc);
        let updated_at = chrono::DateTime::parse_from_rfc3339(&updated_at)?
            .with_timezone(&chrono::Utc);

        Ok(SecureNote {
            title,
            content,
            category,
            created_at,
            updated_at,
        })
    }

    pub fn list_notes(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT title FROM notes ORDER BY title")?;
        let titles = stmt.query_map([], |row| row.get::<_, String>(0))?;

        let mut result = Vec::new();
        for title in titles {
            result.push(title?);
        }

        Ok(result)
    }

    pub fn delete_note(&mut self, title: &str) -> Result<()> {
        self.conn.execute("DELETE FROM notes WHERE title = ?", [title])?;
        Ok(())
    }
}