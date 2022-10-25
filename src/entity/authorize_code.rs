use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct AuthorizeCode {
    pub code: String,
    pub user_id: u32,
    pub client_id: String,
    pub revoked: u32,
    pub expires_at: NaiveDateTime,
}
