use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use crate::entity::access_token::AccessToken;

pub async fn find_by_refresh_token(
    token: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<AccessToken> {
    let refresh_token = sqlx::query_as::<_, AccessToken>(
        r#"
SELECT
    token, user_id, client_id, scope, expires_at
FROM
    access_tokens
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
