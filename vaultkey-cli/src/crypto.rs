use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use anyhow::Result;
use sha2::{Sha256, Digest};
use zeroize::Zeroize;

pub const KEY_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;

/// Derive an encryption key from the hardware key's HMAC output
#[allow(dead_code)]
pub fn derive_key_from_hmac(hmac_output: &[u8]) -> [u8; KEY_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(b"vaultkey-encryption-key-v1");
    hasher.update(hmac_output);
    let result = hasher.finalize();

    let mut key = [0u8; KEY_SIZE];
    key.copy_from_slice(&result);
    key
}

/// Encrypt data with ChaCha20-Poly1305
pub fn encrypt(data: &[u8], key: &[u8; KEY_SIZE]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(key.into());

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt data with ChaCha20-Poly1305
pub fn decrypt(data: &[u8], key: &[u8; KEY_SIZE]) -> Result<Vec<u8>> {
    if data.len() < NONCE_SIZE {
        anyhow::bail!("Data too short to contain nonce");
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(key.into());
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    Ok(plaintext)
}

/// Zeroize sensitive data when dropped
#[allow(dead_code)]
pub struct SensitiveData(pub Vec<u8>);

impl Drop for SensitiveData {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}