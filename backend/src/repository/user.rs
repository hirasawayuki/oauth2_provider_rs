use async_trait::async_trait;
use sqlx::MySqlPool;

use crate::{db::establish_connection, entity::user::User};

pub struct MySqlUserRepository {
    connection_pool: MySqlPool,
}

impl MySqlUserRepository {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = establish_connection().await?;
        anyhow::Ok(Self{connection_pool: pool})
    }
}

#[async_trait]
pub trait UserRepository {
    async fn create(&self, name: &str, email: &str, password_hash: &str) -> anyhow::Result<()>;
    async fn find_by_email(&self, email: &str) -> anyhow::Result<User>;
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(&self, name: &str, email: &str, password_hash: &str) -> anyhow::Result<()> {
        sqlx::query(
            r#"
INSERT INTO users (name, email, password) VALUES (?, ?, ?);
            "#)
            .bind(name)
            .bind(email)
            .bind(String::from(password_hash))
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }

    async fn find_by_email(&self, email: &str) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
SELECT id, name, email, password FROM users WHERE email = ?
            "#)
            .bind(email)
            .fetch_one(&self.connection_pool)
            .await?;

        anyhow::Ok(user)
    }
}
