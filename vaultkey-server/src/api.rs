use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use crate::{AppState, error::AppError};
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecretData {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteData {
    pub title: String,
    pub content: String,
    pub category: Option<String>,
}

// Helper to get user_id from request
fn get_user_id(req: &HttpRequest) -> Result<String, AppError> {
    req.extensions()
        .get::<String>()
        .cloned()
        .ok_or(AppError::Unauthorized)
}

pub async fn list_secrets(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    // Create table if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_secrets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            url TEXT,
            notes TEXT,
            category TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(user_id, name)
        )"
    )
    .execute(&state.db)
    .await?;

    let rows = sqlx::query(
        "SELECT name, username, password, url, notes, category FROM user_secrets WHERE user_id = ? ORDER BY name"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await?;

    let mut secrets = Vec::new();
    for row in rows {
        secrets.push(SecretData {
            name: row.get("name"),
            username: row.get("username"),
            password: row.get("password"),
            url: row.get("url"),
            notes: row.get("notes"),
            category: row.get("category"),
        });
    }

    Ok(HttpResponse::Ok().json(secrets))
}

pub async fn add_secret(
    state: web::Data<AppState>,
    req: HttpRequest,
    secret: web::Json<SecretData>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;
    let now = chrono::Utc::now().to_rfc3339();

    // Create table if not exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_secrets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            url TEXT,
            notes TEXT,
            category TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(user_id, name)
        )"
    )
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT OR REPLACE INTO user_secrets (user_id, name, username, password, url, notes, category, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(&secret.name)
    .bind(&secret.username)
    .bind(&secret.password)
    .bind(&secret.url)
    .bind(&secret.notes)
    .bind(&secret.category)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(secret.into_inner()))
}

pub async fn get_secret(
    state: web::Data<AppState>,
    req: HttpRequest,
    name: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    let row = sqlx::query(
        "SELECT name, username, password, url, notes, category FROM user_secrets WHERE user_id = ? AND name = ?"
    )
    .bind(&user_id)
    .bind(name.as_str())
    .fetch_optional(&state.db)
    .await?;

    match row {
        Some(row) => {
            let secret = SecretData {
                name: row.get("name"),
                username: row.get("username"),
                password: row.get("password"),
                url: row.get("url"),
                notes: row.get("notes"),
                category: row.get("category"),
            };
            Ok(HttpResponse::Ok().json(secret))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_secret(
    state: web::Data<AppState>,
    req: HttpRequest,
    name: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    sqlx::query("DELETE FROM user_secrets WHERE user_id = ? AND name = ?")
        .bind(&user_id)
        .bind(name.as_str())
        .execute(&state.db)
        .await?;

    Ok(HttpResponse::Ok().json("Secret deleted"))
}

pub async fn list_notes(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            category TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(user_id, title)
        )"
    )
    .execute(&state.db)
    .await?;

    let rows = sqlx::query(
        "SELECT title, content, category FROM user_notes WHERE user_id = ? ORDER BY title"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await?;

    let mut notes = Vec::new();
    for row in rows {
        notes.push(NoteData {
            title: row.get("title"),
            content: row.get("content"),
            category: row.get("category"),
        });
    }

    Ok(HttpResponse::Ok().json(notes))
}

pub async fn add_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    note: web::Json<NoteData>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            category TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            UNIQUE(user_id, title)
        )"
    )
    .execute(&state.db)
    .await?;

    sqlx::query(
        "INSERT OR REPLACE INTO user_notes (user_id, title, content, category, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&user_id)
    .bind(&note.title)
    .bind(&note.content)
    .bind(&note.category)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(note.into_inner()))
}

pub async fn get_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    title: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    let row = sqlx::query(
        "SELECT title, content, category FROM user_notes WHERE user_id = ? AND title = ?"
    )
    .bind(&user_id)
    .bind(title.as_str())
    .fetch_optional(&state.db)
    .await?;

    match row {
        Some(row) => {
            let note = NoteData {
                title: row.get("title"),
                content: row.get("content"),
                category: row.get("category"),
            };
            Ok(HttpResponse::Ok().json(note))
        }
        None => Err(AppError::NotFound),
    }
}

pub async fn delete_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    title: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let user_id = get_user_id(&req)?;

    sqlx::query("DELETE FROM user_notes WHERE user_id = ? AND title = ?")
        .bind(&user_id)
        .bind(title.as_str())
        .execute(&state.db)
        .await?;

    Ok(HttpResponse::Ok().json("Note deleted"))
}