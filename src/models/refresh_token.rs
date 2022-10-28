use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use crate::entity::refresh_token::RefreshToken;

pub async fn find_by_refresh_token(
    token: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<RefreshToken> {
    let refresh_token = sqlx::query_as::<_, RefreshToken>(
        r#"
SELECT
    token, access_token, expires_at
FROM
    refresh_tokens
WHERE
    token = ?
        "#)
        .bind(token)
        .fetch_one(connection_pool)
        .await?;

    anyhow::Ok(refresh_token)
}

pub async fn create(
    token: &str,
    access_token: &str,
    expires_at: NaiveDateTime,
    connection_pool: &MySqlPool)
-> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO
    refresh_tokens (token, access_token, expires_at)
VALUES
    (?, ?, ?);
        "#)
        .bind(token)
        .bind(access_token)
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
    refresh_tokens
WHERE
    token = ?;
        "#)
        .bind(token)
        .execute(connection_pool)
        .await?;

    Ok(())
}
