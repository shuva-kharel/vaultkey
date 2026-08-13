use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use webauthn_rs::prelude::*;
use uuid::Uuid;
use crate::{AppState, error::AppError};
use sqlx::Row;
use base64::Engine;

#[derive(Deserialize)]
pub struct RegisterStartRequest {
    pub username: String,
}

#[derive(Serialize)]
pub struct RegisterStartResponse {
    pub challenge: CreationChallengeResponse,
    pub user_id: String,
}

#[derive(Deserialize)]
pub struct RegisterFinishRequest {
    pub username: String,
    pub user_id: String,
    #[allow(dead_code)]
    pub registration: RegisterPublicKeyCredential,
}

#[derive(Deserialize)]
pub struct LoginStartRequest {
    pub username: String,
}

#[derive(Deserialize)]
pub struct LoginFinishRequest {
    pub username: String,
    #[allow(dead_code)]
    pub authentication: PublicKeyCredential,
}

#[derive(Serialize)]
pub struct LoginFinishResponse {
    pub token: String,
}

pub async fn register_start(
    state: web::Data<AppState>,
    req: web::Json<RegisterStartRequest>,
) -> Result<HttpResponse, AppError> {
    // Check if username already exists
    let existing = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_optional(&state.db)
        .await?;
    if existing.is_some() {
        return Err(AppError::BadRequest("Username already taken".into()));
    }

    let user_id = Uuid::new_v4();

    let (challenge, reg_state) = state.webauthn.start_passkey_registration(
        user_id,
        &req.username,
        &req.username,
        None,
    )?;

    // Store reg_state in memory
    {
        let mut pending = state.pending_registrations.lock().unwrap();
        pending.insert(user_id.to_string(), reg_state);
    }

    Ok(HttpResponse::Ok().json(RegisterStartResponse {
        challenge,
        user_id: user_id.to_string(),
    }))
}

pub async fn register_finish(
    state: web::Data<AppState>,
    req: web::Json<RegisterFinishRequest>,
) -> Result<HttpResponse, AppError> {
    let user_id = Uuid::parse_str(&req.user_id)
        .map_err(|_| AppError::BadRequest("Invalid user_id".into()))?;

    // For now, skip WebAuthn validation and just store the user
    // In production, we'd validate the registration properly

    let user_uuid_str = user_id.to_string();

    // Check if user already exists
    let existing = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_optional(&state.db)
        .await?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Username already exists".into()));
    }

    // Insert user
    sqlx::query("INSERT INTO users (id, username) VALUES (?, ?)")
        .bind(&user_uuid_str)
        .bind(&req.username)
        .execute(&state.db)
        .await?;

    // Store a dummy credential for now
    sqlx::query(
        "INSERT INTO credentials (id, user_id, passkey_data, sign_count) VALUES (?, ?, ?, ?)"
    )
    .bind("dummy-credential-id")
    .bind(&user_uuid_str)
    .bind(vec![0u8; 32])  // Dummy passkey data
    .bind(0i64)
    .execute(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json("Registration successful"))
}

pub async fn login_start(
    state: web::Data<AppState>,
    req: web::Json<LoginStartRequest>,
) -> Result<HttpResponse, AppError> {
    // Fetch user
    let user_row = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_optional(&state.db)
        .await?;

    // Just check if user exists (we don't need the id yet)
    match user_row {
        Some(_) => {
            // For now, return a simple challenge
            // In production, this would be a proper WebAuthn challenge
            let challenge = serde_json::json!({
                "publicKey": {
                    "challenge": base64::engine::general_purpose::STANDARD.encode(b"test-challenge"),
                    "rpId": "localhost",
                    "allowCredentials": [],
                    "userVerification": "required"
                }
            });

            Ok(HttpResponse::Ok().json(challenge))
        }
        None => Err(AppError::Unauthorized),
    }
}

pub async fn login_finish(
    state: web::Data<AppState>,
    req: web::Json<LoginFinishRequest>,
) -> Result<HttpResponse, AppError> {
    // Fetch user
    let user_row = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&req.username)
        .fetch_optional(&state.db)
        .await?;

    let user_id = match user_row {
        Some(row) => row.get::<String, _>("id"),
        None => return Err(AppError::Unauthorized),
    };

    // Generate JWT
    let token = create_jwt(&state.jwt_secret, &user_id, state.jwt_expiration_hours)?;

    Ok(HttpResponse::Ok().json(LoginFinishResponse { token }))
}

// JWT utility
fn create_jwt(secret: &[u8], user_id: &str, expiration_hours: i64) -> Result<String, AppError> {
    use jsonwebtoken::{encode, Header, EncodingKey};
    use chrono::{Utc, Duration};

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        exp: usize,
    }

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now() + Duration::hours(expiration_hours)).timestamp() as usize,
    };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(token)
}