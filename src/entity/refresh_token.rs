use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct RefreshToken {
    pub token: String,
    pub access_token: String,
    pub expires_at: NaiveDateTime,
}
