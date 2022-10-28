use sqlx::MySqlPool;

use crate::entity::oauth_client::OAuthClient;

pub async fn create(
    client_id: &str,
    client_secret: &str,
    name: &str,
    user_id: &str,
    redirect_uri: &str,
    scope: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
INSERT INTO
    oauth_clients (client_id, client_secret, name, user_id, redirect_uri, scope)
VALUES
    (?, ?, ?, ?, ?, ?);
        "#)
        .bind(client_id)
        .bind(client_secret)
        .bind(name)
        .bind(user_id)
        .bind(redirect_uri)
        .bind(scope)
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
    client_id, client_secret, name, user_id, scope, redirect_uri
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

pub async fn find_by_user_id(
    user_id: &str,
    connection_pool: &MySqlPool
) -> anyhow::Result<Vec<OAuthClient>> {
    let oauth_clients = sqlx::query_as::<_, OAuthClient>(
        r#"
SELECT
    client_id, client_secret, name, user_id, scope, redirect_uri
FROM
    oauth_clients
WHERE
    user_id = ?
        "#)
        .bind(user_id)
        .fetch_all(connection_pool)
        .await?;

    anyhow::Ok(oauth_clients)
}
