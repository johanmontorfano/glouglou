use super::{generate_api_key, APIKeyData, AES_KEY, API_KEY_LENGTH, GG_KEY_STORAGE_PATH};
use crate::utils::{fs::raw_read_file, log::Log};
use aes_gcm::{
    aead::{generic_array::GenericArray, AeadMut},
    Aes256Gcm, KeyInit, Nonce,
};
use std::{
    fs,
    time::{Duration, SystemTime},
};
use tokio::time::interval;

// Uses `secure_key_setup` on a routine, the function is ran every hour.
pub async fn sks_routine() {
    let mut interval = interval(Duration::from_secs(1800));

    loop {
        secure_key_setup();
        interval.tick().await;
        interval.tick().await;
    }
}

// Securely setup and checks API keys for updates or creation.
pub fn secure_key_setup() {
    let log = Log::new("🔑".into());

    if let Ok(_) = raw_read_file(GG_KEY_STORAGE_PATH) {
        // Decrypt file content and compares the creation timestamp with the current time.
        // If the timestamp is at least of 30 days (in milliseconds), the key is regen.
        let api_key = APIKeyData::load_api_key().unwrap();
        let run_timestamp = SystemTime::now().elapsed().unwrap().as_millis();

        if run_timestamp - api_key.creation_timestamp
            > Duration::from_millis(2592000000).as_millis()
        {
            setup_key(true).expect("With a too old API key, the service cannot work");
            log.out(format!("API Key health: Too old"));
        } else {
            log.out(format!("API Key health: OK"));
        }
    } else {
        // If the file doesn't exists, a new API Key is created and a file is populated.
        setup_key(true).expect("With no API keys, the service cannot work");
    }
}

// Setups a new key without checking if `GG_KEY_STORAGE_PATH` exists or not as it's overwritten.
pub fn setup_key(allow_panic: bool) -> Result<(), ()> {
    let log = Log::new("setup_key".into());

    let api_key = generate_api_key(API_KEY_LENGTH.into());
    let gen_timestamp = SystemTime::now().elapsed().unwrap().as_millis();

    let crypto_key = GenericArray::from_iter(AES_KEY.bytes());
    let mut cipher = Aes256Gcm::new(&crypto_key);
    let nonce = Nonce::from_slice(b"aesgcm_sk_no");

    let crypto_output = cipher.encrypt(
        nonce,
        format!("{} {}", api_key, gen_timestamp).as_bytes().as_ref(),
    );
    match crypto_output {
        Ok(bytes) => {
            if fs::write(GG_KEY_STORAGE_PATH, bytes).is_err() {
                if allow_panic {
                    log.panic("Cannot setup a new API key.");
                }
                return Err(());
            } else {
                log.out(format!("New API key: {}", api_key));
            }
            Ok(())
        }
        Err(reason) => {
            if allow_panic {
                log.panic(format!("Cannot setup a new API key: {}", reason))
            }
            Err(())
        }
    }
}
