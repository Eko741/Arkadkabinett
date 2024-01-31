
use crate::SharedMem;

use rsa::RsaPrivateKey;
use::rsa::{Oaep, sha2::Sha256};

use base64::{engine::general_purpose, Engine as _};

use crate::util::find_val;
// Decrypts a value from the request header
pub fn decrypt_header(request_header: &Vec<String>, shared_mem: &std::sync::Arc<SharedMem>, header: &str) -> Result<String, String> {

    // Finds the encrypted message in the request header
    let m_encrypted_base64: String = match find_val(request_header, header) {
        Some(m) => m,
        None => return Err("No key".to_string())
    };

    // Decrypts the base64 encoded RSA encrypted message  
    let m_decrypted_string = match decrypt_base64(m_encrypted_base64, &shared_mem.private_key) {
        Ok(s) => s,
        Err(err) => return Err(err)
    };

    println!("{m_decrypted_string}");

    Ok(m_decrypted_string)
}

pub fn decrypt_base64(m_encrypted_base64: String, private_key: &RsaPrivateKey) -> Result<String, String>{
    
    // Decodes the key from base64 to Vec of bytes
    let m_encrypted_bytes: Vec<u8> = match general_purpose::STANDARD.decode(m_encrypted_base64) {
        Ok(m) => m,
        Err(err) => return Err(err.to_string())
    };

    // Decrypts the key with the private key and padding
    let padding = Oaep::new::<Sha256>();
    let m_decrypted_bytes = match private_key.decrypt(padding, &m_encrypted_bytes){
        Ok(d) => d,
        Err(err) => return Err(err.to_string())
    };

    // Converts the Vec of bytes to a string 
    match String::from_utf8(m_decrypted_bytes) {
        Ok(s) => Ok(s),
        Err(err) => return Err(err.to_string())
    }
}