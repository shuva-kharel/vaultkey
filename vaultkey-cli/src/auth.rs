use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use base64::Engine;
use crate::config::Config;
use crate::hardware_key::HardwareKey;

#[derive(Debug, Serialize, Deserialize)]
struct RegisterStartRequest {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterStartResponse {
    challenge: serde_json::Value,
    user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginStartRequest {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginFinishResponse {
    token: String,
}

pub async fn register(
    _config: &Config,
    hw_key: &HardwareKey,
    username: &str,
    server: &str,
) -> Result<()> {
    let client = reqwest::Client::new();

    // Step 1: Start registration
    println!("[*] Starting registration...");
    let start_resp: RegisterStartResponse = client
        .post(format!("{}/register/start", server))
        .json(&RegisterStartRequest {
            username: username.to_string(),
        })
        .send()
        .await?
        .json()
        .await?;

    // Step 2: Perform WebAuthn registration with hardware key
    println!("[*] Please touch your hardware key...");

    // Extract challenge from response
    let challenge_json = &start_resp.challenge;
    let challenge_str = challenge_json["publicKey"]["challenge"]
        .as_str()
        .context("Invalid challenge")?;

    // Decode base64url challenge
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(challenge_str)?;

    // Perform registration
    let credential = hw_key.register_credential(
        &challenge,
        "localhost",
        start_resp.user_id.as_bytes(),
        username,
    )?;

    // Step 3: Complete registration with server
    println!("[*] Completing registration...");
    let _finish_resp: serde_json::Value = client
        .post(format!("{}/register/finish", server))
        .json(&serde_json::json!({
            "username": username,
            "user_id": start_resp.user_id,
            "registration": credential,
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("[✓] Registration successful");
    Ok(())
}

pub async fn login(
    _config: &Config,
    hw_key: &HardwareKey,
    username: &str,
    server: &str,
) -> Result<String> {
    let client = reqwest::Client::new();

    // Step 1: Start login
    println!("[*] Starting login...");
    let start_resp: serde_json::Value = client
        .post(format!("{}/login/start", server))
        .json(&LoginStartRequest {
            username: username.to_string(),
        })
        .send()
        .await?
        .json()
        .await?;

    // Debug: print response
    println!("[DEBUG] Login start response: {}", start_resp);

    // Step 2: Perform WebAuthn authentication
    println!("[*] Please touch your hardware key...");

    // Extract challenge
    let challenge_str = start_resp["publicKey"]["challenge"]
        .as_str()
        .or_else(|| start_resp["challenge"].as_str())
        .context("Invalid challenge")?;

    // Try different base64 decodings
    let challenge = base64::engine::general_purpose::STANDARD
        .decode(challenge_str)
        .or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(challenge_str))
        .or_else(|_| base64::engine::general_purpose::URL_SAFE.decode(challenge_str))
        .context("Could not decode challenge")?;

    let credential = hw_key.authenticate(
        &challenge,
        "localhost",
        b"dummy-credential-id",
    )?;

    // Step 3: Complete login
    println!("[*] Completing login...");
    let finish_resp: LoginFinishResponse = client
        .post(format!("{}/login/finish", server))
        .json(&serde_json::json!({
            "username": username,
            "authentication": credential,
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(finish_resp.token)
}