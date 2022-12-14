use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct OAuthClient {
    pub client_id: String,
    pub client_secret: String,
    pub name: String,
    pub user_id: u32,
    pub scope: String,
    pub redirect_uri: String,
}
