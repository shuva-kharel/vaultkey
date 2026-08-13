use actix_web::{web, HttpResponse, HttpRequest, HttpMessage};
use serde::Deserialize;
use sqlx::Row;
use crate::{AppState, error::AppError};

#[derive(Deserialize)]
pub struct PutVaultRequest {
    pub data: Vec<u8>,
    pub version: Option<i64>,
}

pub async fn put_vault(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<PutVaultRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = req.extensions().get::<String>().cloned()
        .ok_or(AppError::Unauthorized)?;

    let storage_root = &state.storage_root;
    let file_name = format!("{}.vault", user_id);
    let file_path = storage_root.join(&file_name);

    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    tokio::fs::write(&file_path, &payload.data).await?;

    let version = payload.version.unwrap_or(1);
    sqlx::query(
        r#"
        INSERT INTO vaults (user_id, file_path, version, updated_at)
        VALUES (?, ?, ?, CURRENT_TIMESTAMP)
        ON CONFLICT(user_id) DO UPDATE SET
            file_path = excluded.file_path,
            version = excluded.version,
            updated_at = CURRENT_TIMESTAMP
        "#,
    )
    .bind(&user_id)
    .bind(file_name)
    .bind(version)
    .execute(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json("Vault saved"))
}

pub async fn get_vault(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = req.extensions().get::<String>().cloned()
        .ok_or(AppError::Unauthorized)?;

    let row = sqlx::query("SELECT file_path FROM vaults WHERE user_id = ?")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await?;

    let file_name: String = match row {
        Some(r) => r.get("file_path"),
        None => return Err(AppError::NotFound),
    };

    let file_path = state.storage_root.join(file_name);
    let bytes = tokio::fs::read(&file_path).await?;

    Ok(HttpResponse::Ok().content_type("application/octet-stream").body(bytes))
}

pub async fn delete_vault(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user_id = req.extensions().get::<String>().cloned()
        .ok_or(AppError::Unauthorized)?;

    let row = sqlx::query("SELECT file_path FROM vaults WHERE user_id = ?")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await?;

    if let Some(r) = row {
        let file_name: String = r.get("file_path");
        let file_path = state.storage_root.join(file_name);
        let _ = tokio::fs::remove_file(&file_path).await;
    }

    sqlx::query("DELETE FROM vaults WHERE user_id = ?")
        .bind(&user_id)
        .execute(&state.db)
        .await?;

    Ok(HttpResponse::Ok().json("Vault deleted"))
}