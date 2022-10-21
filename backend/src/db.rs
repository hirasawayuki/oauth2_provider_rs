use dotenv::dotenv;
use std::env;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};


pub async fn establish_connection() -> anyhow::Result<MySqlPool> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(1))
        .connect(&db_url)
        .await?;

    anyhow::Ok(pool)
}
