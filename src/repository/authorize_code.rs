use chrono::NaiveDateTime;
use sqlx::MySqlPool;

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
    authorize_codes (code, user_id, client_id, expires_at)
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
