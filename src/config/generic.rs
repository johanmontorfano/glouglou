use serde_derive::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GenericConfiguration  {
    pub email: GenConfEmail,
    pub server: GenConfServer,
    pub dkim: Option<GenConfDkim>
}

// Configuration of the email address to use.
#[derive(Deserialize, Clone)]
pub struct GenConfEmail {
    pub address: String,
    pub password: String,
    pub host: String,
    pub port: u32
}

// Configuration of the server for the REST API.
#[derive(Deserialize, Clone)]
pub struct GenConfServer {
    pub http_port: u16
}

// Configuration of the DKIM encryption.
#[derive(Deserialize, Clone)]
pub struct GenConfDkim {
    pub domain: String,
    pub selector: String,
    pub private_key_path: String,
    pub expiration: u32
}
