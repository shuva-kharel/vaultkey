mod config;
mod db;
mod error;
mod middleware;
mod vault;
mod webauthn;

use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use webauthn_rs::prelude::*;
use url::Url;

pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub webauthn: Arc<Webauthn>,
    pub storage_root: std::path::PathBuf,
    pub jwt_secret: Vec<u8>,
    pub jwt_expiration_hours: i64,
    pub pending_registrations: Mutex<HashMap<String, PasskeyRegistration>>,
    pub pending_authentications: Mutex<HashMap<String, PasskeyAuthentication>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::load()?;
    std::fs::create_dir_all(&config.storage.root)?;

    let db = db::init_pool(&config.database.url).await?;
    db::run_migrations(&db).await?;

    // WebAuthn setup
    let rp_id = config.webauthn.rp_id.clone();
    let rp_origin = Url::parse(&config.webauthn.rp_origin)?;
    let webauthn = Arc::new(
        WebauthnBuilder::new(&rp_id, &rp_origin)?
            .rp_name("Vaultkey")
            .build()?,
    );

    let state = AppState {
        db,
        webauthn,
        storage_root: config.storage.root,
        jwt_secret: config.jwt.secret.into_bytes(),
        jwt_expiration_hours: config.jwt.expiration_hours,
        pending_registrations: Mutex::new(HashMap::new()),
        pending_authentications: Mutex::new(HashMap::new()),
    };

    let state = web::Data::new(state);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(actix_middleware::Logger::default())
            .route("/register/start", web::post().to(webauthn::register_start))
            .route("/register/finish", web::post().to(webauthn::register_finish))
            .route("/login/start", web::post().to(webauthn::login_start))
            .route("/login/finish", web::post().to(webauthn::login_finish))
            .service(
                web::scope("/vault")
                    .wrap(middleware::JwtAuth)
                    .route("", web::get().to(vault::get_vault))
                    .route("", web::put().to(vault::put_vault))
                    .route("", web::delete().to(vault::delete_vault)),
            )
            .route("/health", web::get().to(|| async { "ok" }))
    })
    .bind((config.server.listen_addr.as_str(), config.server.listen_port))?
    .run()
    .await?;

    Ok(())
}