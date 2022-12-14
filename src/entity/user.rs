use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
}
