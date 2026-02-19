use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use sha2::{Sha256, Digest};
use std::env;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub fn get_key() -> [u8; 32] {
    let key_str = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");

    // Derive a proper 32-byte key using SHA-256 regardless of input format.
    // This ensures full 256-bit keyspace even if ENCRYPTION_KEY is a passphrase.
    let hash = Sha256::digest(key_str.as_bytes());
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash);
    key
}

pub fn encrypt_token(token: &str) -> String {
    let key = get_key();
    let cipher = Aes256Gcm::new(&key.into());
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce = Nonce::from(nonce);

    let ciphertext = cipher.encrypt(&nonce, token.as_bytes()).expect("encryption failure");
    
    // Return format: nonce(12 bytes) + ciphertext (base64 encoded together or separate)
    // Easier: base64(nonce + ciphertext)
    let mut combined = nonce.to_vec();
    combined.extend(ciphertext);
    
    BASE64.encode(combined)
}

pub fn decrypt_token(encrypted_data: &str) -> Option<String> {
    let data = BASE64.decode(encrypted_data).ok()?;
    if data.len() < 12 {
        return None;
    }
    
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let key = get_key();
    let cipher = Aes256Gcm::new(&key.into());

    let plaintext = cipher.decrypt(nonce, ciphertext).ok()?;
    String::from_utf8(plaintext).ok()
}
