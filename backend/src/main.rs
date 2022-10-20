use std::env;
use actix_web::{HttpServer, App, web};
use anyhow::{Result, Ok};
use dotenv::dotenv;

mod db;
mod handler;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")?;
    let pool = db::init_pool(&db_url).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/").route(web::get().to(handler::top::index)))
            .service(web::resource("/signup").route(web::get().to(handler::signup::new)))
            .service(web::resource("/register").route(web::post().to(handler::signup::create)))
            .service(web::resource("/login").route(web::get().to(handler::login::new)))
            .service(web::resource("/authenticate").route(web::post().to(handler::login::create)))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
