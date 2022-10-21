use serde::{Deserialize, Serialize};

use crate::utils::hash_password::verify_password;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    password: String,
}

impl User {
    pub fn verify_password(&self, password: &str) -> anyhow::Result<bool> {
        let result = verify_password(&self.password, password)?;

        return anyhow::Ok(result);
    }
}
