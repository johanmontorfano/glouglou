pub const GG_KEY_STORAGE_PATH: &str = "./gg.skey";
pub const API_KEY_LENGTH: u16 = 256;
pub const AES_KEY: &str = "aDsk_1.;a@!Mf,e*d^péà94@d..din";

mod gen;
pub use gen::generate_api_key;

mod setup;
pub use setup::{secure_key_setup, sks_routine};

mod load;
pub use load::APIKeyData;