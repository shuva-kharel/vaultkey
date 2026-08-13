use anyhow::{Result, Context};
use sha2::{Sha256, Digest};
use base64::Engine;
use std::path::PathBuf;

pub struct HardwareKey {
    key_id: String,
    secret: [u8; 32],
}

impl HardwareKey {
    pub fn new() -> Result<Self> {
        // Store the simulated key in a file so it persists between runs
        let key_path = get_key_path()?;

        let secret = if key_path.exists() {
            // Load existing key
            let key_bytes = std::fs::read(&key_path)?;
            let mut secret = [0u8; 32];
            secret.copy_from_slice(&key_bytes);
            secret
        } else {
            // Generate new key and save it
            let mut secret = [0u8; 32];
            rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut secret);

            // Save to file
            if let Some(parent) = key_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&key_path, &secret)?;

            secret
        };

        Ok(HardwareKey {
            key_id: "Simulated Key (Development)".to_string(),
            secret,
        })
    }

    pub fn get_key_id(&self) -> Result<String> {
        Ok(self.key_id.clone())
    }

    /// Derive a stable encryption key
    pub fn derive_key(&self) -> Result<[u8; 32]> {
        println!("[*] Please touch your hardware key...");

        // Derive key using SHA-256 for stability
        let mut hasher = Sha256::new();
        hasher.update(b"vaultkey-master-key-derivation-v1");
        hasher.update(&self.secret);
        let result = hasher.finalize();

        let mut key = [0u8; 32];
        key.copy_from_slice(&result);

        Ok(key)
    }

    /// Simulate WebAuthn registration
    pub fn register_credential(
        &self,
        challenge: &[u8],
        rp_id: &str,
        _user_id: &[u8],
        user_name: &str,
    ) -> Result<serde_json::Value> {
        println!("[*] Simulating WebAuthn registration...");
        println!("[*] Challenge: {}", base64::engine::general_purpose::STANDARD.encode(challenge));
        println!("[*] Relying Party: {}", rp_id);
        println!("[*] User: {}", user_name);

        // Generate a simulated credential
        let credential_id = self.generate_credential_id();

        Ok(serde_json::json!({
            "id": base64::engine::general_purpose::STANDARD.encode(&credential_id),
            "rawId": base64::engine::general_purpose::STANDARD.encode(&credential_id),
            "type": "public-key",
            "response": {
                "clientDataJSON": base64::engine::general_purpose::STANDARD.encode(b"{}"),
                "attestationObject": base64::engine::general_purpose::STANDARD.encode(b"")
            }
        }))
    }

    /// Simulate WebAuthn authentication
    pub fn authenticate(
        &self,
        challenge: &[u8],
        rp_id: &str,
        credential_id: &[u8],
    ) -> Result<serde_json::Value> {
        println!("[*] Simulating WebAuthn authentication...");
        println!("[*] Challenge: {}", base64::engine::general_purpose::STANDARD.encode(challenge));
        println!("[*] Relying Party: {}", rp_id);

        // Generate simulated assertion
        Ok(serde_json::json!({
            "id": base64::engine::general_purpose::STANDARD.encode(credential_id),
            "rawId": base64::engine::general_purpose::STANDARD.encode(credential_id),
            "type": "public-key",
            "response": {
                "clientDataJSON": base64::engine::general_purpose::STANDARD.encode(b"{}"),
                "authenticatorData": base64::engine::general_purpose::STANDARD.encode(b""),
                "signature": base64::engine::general_purpose::STANDARD.encode(b""),
                "userHandle": null
            }
        }))
    }

    fn generate_credential_id(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.secret);
        hasher.update(b"credential-id");
        hasher.finalize().to_vec()
    }
}

fn get_key_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not find config directory")?
        .join("vaultkey");

    std::fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("hardware_key.bin"))
}