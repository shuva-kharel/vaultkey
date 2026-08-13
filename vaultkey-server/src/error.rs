use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("WebAuthn error: {0}")]
    Webauthn(#[from] webauthn_rs_core::error::WebauthnError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid request: {0}")]
    BadRequest(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::BadRequest(msg) => HttpResponse::BadRequest().json(msg),
            AppError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            AppError::NotFound => HttpResponse::NotFound().json("Not found"),
            _ => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}