use chrono::NaiveDateTime;
use sqlx::MySqlPool;

pub async fn create(
    token: &str,
    user_id: &str,
    client_id: &str,
    scope: &str,
    expires_at: NaiveDateTime,
    connection_pool: &MySqlPool)
-> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO
    access_tokens (token, user_id, client_id, scope, expires_at)
VALUES
    (?, ?, ?, ?, ?);
        "#)
        .bind(token)
        .bind(user_id)
        .bind(client_id)
        .bind(scope)
        .bind(expires_at.to_string())
        .execute(connection_pool)
        .await?;

    Ok(())
}

pub async fn delete(
    token: &str,
    connection_pool: &MySqlPool
)-> anyhow::Result<()> {
    sqlx::query(
        r#"
DELETE FROM
    access_tokens
WHERE
    token = ?;
        "#)
        .bind(token)
        .execute(connection_pool)
        .await?;

    Ok(())
}
