use sqlx::MySqlPool;

pub async fn create(
    user_id: &str,
    name: &str,
    redirect_uri: &str,
    scope: &str,
    client_id: &str,
    client_secret: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO
    oauth_clients (user_id, name, redirect_uri, scope, client_id, client_secret)
VALUES
    (?, ?, ?, ?, ?, ?);
        "#)
        .bind(user_id)
        .bind(name)
        .bind(redirect_uri)
        .bind(scope)
        .bind(client_id)
        .bind(client_secret)
        .execute(connection_pool)
        .await?;
    Ok(())
}
