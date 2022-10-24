use sqlx::MySqlPool;
use crate::entity::user::User;

pub async fn create(name: &str, email: &str, password_hash: &str, connection_pool: &MySqlPool) -> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO users (name, email, password) VALUES (?, ?, ?);
        "#)
        .bind(name)
        .bind(email)
        .bind(String::from(password_hash))
        .execute(connection_pool)
        .await?;
    Ok(())
}

pub async fn find_by_email(email: &str, connection_pool: &MySqlPool) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
SELECT id, name, email, password FROM users WHERE email = ?
        "#)
        .bind(email)
        .fetch_one(connection_pool)
        .await?;

    anyhow::Ok(user)
}
