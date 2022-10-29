use chrono::NaiveDateTime;
use sqlx::{ MySqlExecutor, MySql};

use crate::entity::refresh_token::RefreshToken;

pub async fn find_by_refresh_token<'e, E>(
    token: &str,
    executor: E
) -> anyhow::Result<RefreshToken>
where
    E: MySqlExecutor<'e, Database = MySql>
{
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
        .fetch_one(executor)
        .await?;

    anyhow::Ok(refresh_token)
}

pub async fn create<'e, E>(
    token: &str,
    access_token: &str,
    expires_at: NaiveDateTime,
    executor: E
) -> anyhow::Result<()>
where
    E: MySqlExecutor<'e, Database = MySql>
{
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
        .execute(executor)
        .await?;

    Ok(())
}

pub async fn delete<'e, E>(
    token: &str,
    executor: E
)-> anyhow::Result<()>
where E: MySqlExecutor<'e, Database = MySql>
{
    sqlx::query(
        r#"
DELETE FROM
    refresh_tokens
WHERE
    token = ?;
        "#)
        .bind(token)
        .execute(executor)
        .await?;

    Ok(())
}
