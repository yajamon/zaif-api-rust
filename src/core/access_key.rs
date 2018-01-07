#[derive(Clone)]
pub struct AccessKey {
    pub key: String,
    pub secret: String,
}

impl AccessKey {
    pub fn new(key: &str, secret: &str) -> Self {
        AccessKey {
            key: key.to_string(),
            secret: secret.to_string(),
        }
    }
}
