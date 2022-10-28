use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct AccessToken {
    pub token: String,
    pub user_id: u32,
    pub client_id: String,
    pub scope: String,
    pub expires_at: NaiveDateTime,
}
