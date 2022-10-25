use sqlx::MySqlPool;

use crate::entity::oauth_client::OAuthClient;

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

pub async fn find_by_client_id(
    client_id: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<OAuthClient> {
    let oauth_client = sqlx::query_as::<_, OAuthClient>(
        r#"
SELECT
    id, name, user_id, client_id, client_secret, scope, revoked, redirect_uri
FROM
    oauth_clients
WHERE
    client_id = ?
        "#)
        .bind(client_id)
        .fetch_one(connection_pool)
        .await?;

    anyhow::Ok(oauth_client)
}
