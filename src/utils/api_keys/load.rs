use aes_gcm::{
    aead::{generic_array::GenericArray, Aead},
    Aes256Gcm, KeyInit, Nonce,
};
use crate::utils::fs::raw_read_file;

use super::{AES_KEY, GG_KEY_STORAGE_PATH};

// Load data linked to the API key.
#[derive(Clone)]
pub struct APIKeyData {
    pub api_key: String,
    pub creation_timestamp: u128,
}

impl APIKeyData {
    pub fn load_api_key() -> Result<Self, ()> {
        if let Ok(file_content) = raw_read_file(GG_KEY_STORAGE_PATH) {
            // Decrypt file content.
            let crypto_key = GenericArray::from_iter(AES_KEY.bytes());
            let cipher = Aes256Gcm::new(&crypto_key);
            let nonce = Nonce::from_slice(b"aesgcm_sk_no");

            let decfile_content = cipher.decrypt(nonce, file_content.as_ref()).unwrap();
            let decfile_content = String::from_utf8_lossy(decfile_content.as_slice()).to_string();
            let splitted_content = decfile_content.split_once(" ").unwrap();

            Ok(Self {
                api_key: splitted_content.0.into(),
                creation_timestamp: splitted_content.1.parse().unwrap(),
            })
        } else {
            Err(())
        }
    }
}
