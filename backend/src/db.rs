use anyhow::{Result, Ok};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn init_pool(db_url: &str) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(db_url)
        .await?;

    Ok(pool)
}
