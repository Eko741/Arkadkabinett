use std::collections::HashMap;

use crate::{RSAKey, SharedMem};

use rsa::{
    pkcs8::{EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};

use ::rsa::{sha2::Sha256, Oaep};

use base64::{engine::general_purpose, Engine as _};

// Decrypts a value from the request header
pub fn decrypt_header(
    request_header: &HashMap<String, String>,
    shared_mem: &std::sync::Arc<SharedMem>,
    header: &str,
) -> Result<String, String> {
    // Finds the encrypted message in the request header
    let m_encrypted_base64: String = request_header.get(header).unwrap().to_string();

    // Decrypts the base64 encoded RSA encrypted message
    let m_decrypted_string =
        match decrypt_base64(m_encrypted_base64, &shared_mem.rsa_key.private_key) {
            Ok(s) => s,
            Err(err) => return Err(err),
        };

    Ok(m_decrypted_string)
}

pub fn decrypt_base64(
    m_encrypted_base64: String,
    private_key: &RsaPrivateKey,
) -> Result<String, String> {
    // Decodes the key from base64 to Vec of bytes
    let m_encrypted_bytes: Vec<u8> = match general_purpose::STANDARD.decode(m_encrypted_base64) {
        Ok(m) => m,
        Err(err) => return Err(err.to_string()),
    };

    // Decrypts the key with the private key and padding
    let padding = Oaep::new::<Sha256>();
    let m_decrypted_bytes = match private_key.decrypt(padding, &m_encrypted_bytes) {
        Ok(d) => d,
        Err(err) => return Err(err.to_string()),
    };

    // Converts the Vec of bytes to a string
    match String::from_utf8(m_decrypted_bytes) {
        Ok(s) => Ok(s),
        Err(err) => Err(err.to_string()),
    }
}

// Generates a private RSA key, public RSA key and a PEM representation of the public key
pub fn generate_key_pair() -> RSAKey {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    let pub_key_encoded = pub_key
        .to_public_key_pem(LineEnding::CRLF)
        .expect("Failed to encode public key");

    RSAKey {
        public_key: pub_key,
        public_key_encoded: pub_key_encoded,
        private_key: priv_key,
    }
}
