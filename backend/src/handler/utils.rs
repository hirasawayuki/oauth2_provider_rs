use argon2::{Config, hash_encoded};

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let config = Config::default();
    let salt = b"randomsalt";
    let hash = hash_encoded(password.as_bytes(), salt, &config)?;

    return anyhow::Ok(hash);
}

