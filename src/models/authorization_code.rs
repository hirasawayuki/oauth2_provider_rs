use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use crate::entity::authorization_code::AuthorizationCode;

pub async fn find_by_code(
    code: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<AuthorizationCode> {
    let authorization_code = sqlx::query_as::<_, AuthorizationCode>(
        r#"
SELECT
    code, user_id, client_id, expires_at
FROM
    authorization_codes
WHERE
    code = ?
        "#)
        .bind(code)
        .fetch_one(connection_pool)
        .await?;

    anyhow::Ok(authorization_code)
}

pub async fn create(
    code: &str,
    user_id: &str,
    client_id: &str,
    expires_at: NaiveDateTime,
    connection_pool: &MySqlPool)
-> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO
    authorization_codes (code, user_id, client_id, expires_at)
VALUES
    (?, ?, ?, ?);
        "#)
        .bind(code)
        .bind(user_id)
        .bind(client_id)
        .bind(expires_at.to_string())
        .execute(connection_pool)
        .await?;

    Ok(())
}
