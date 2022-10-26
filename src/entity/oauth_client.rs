use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct OAuthClient {
    pub id: u32,
    pub user_id: u32,
    pub client_id: String,
    pub client_secret: String,
    pub name: String,
    pub scope: String,
    pub redirect_uri: String,
}
