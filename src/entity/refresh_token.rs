use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct RefreshToken {
    pub token: String,
    pub access_token: String,
    pub revoked: u32,
    pub expires_at: String,
}
