use chrono::NaiveDateTime;
use sqlx::{ MySqlExecutor, MySql };

use crate::entity::access_token::AccessToken;

pub async fn find_by_refresh_token<'e, E>(
    token: &str,
    executor: E,
) -> anyhow::Result<AccessToken>
where
    E: MySqlExecutor<'e, Database = MySql>
{
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
        .fetch_one(executor)
        .await?;

    anyhow::Ok(refresh_token)
}

pub async fn create<'e, E>(
    token: &str,
    user_id: &str,
    client_id: &str,
    scope: &str,
    expires_at: NaiveDateTime,
    executor: E
) -> anyhow::Result<()>
where
    E: MySqlExecutor<'e, Database = MySql>
{
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
        .execute(executor)
        .await?;

    Ok(())
}

pub async fn delete<'e, E>(
    token: &str,
    executor: E,
)-> anyhow::Result<()>
where
    E: MySqlExecutor<'e, Database = MySql>
{
    sqlx::query(
        r#"
DELETE FROM
    access_tokens
WHERE
    token = ?;
        "#)
        .bind(token)
        .execute(executor)
        .await?;

    Ok(())
}
