use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore, config::PersistentSession};
use actix_web::{HttpServer, App, web, middleware::Logger, cookie::{self, Key}};
use anyhow::{Result, Ok};
use middleware::auth::Authenticator;

mod db;
mod handler;
mod repository;
mod entity;
mod utils;
mod middleware;

#[actix_web::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let connection_pool = db::establish_connection().await?;

    HttpServer::new(move || {
        let session_mw = SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
            .cookie_secure(false)
            .session_lifecycle(PersistentSession::default().session_ttl(cookie::time::Duration::hours(1)))
            .build();

        App::new()
            .app_data(web::Data::new(connection_pool.clone()))
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(session_mw)
            .service(web::resource("/").route(web::get().to(handler::top::index)))
            .service(web::resource("/signup").route(web::get().to(handler::signup::new)))
            .service(web::resource("/register").route(web::post().to(handler::signup::create)))
            .service(web::resource("/oauth_client/new").wrap(Authenticator).route(web::get().to(handler::oauth_client::new)))
            .service(web::resource("/oauth_client/register").wrap(Authenticator).route(web::post().to(handler::oauth_client::create)))
            .service(web::resource("/login").route(web::get().to(handler::login::new_session)))
            .service(web::resource("/logout").route(web::post().to(handler::login::delete_session)))
            .service(web::resource("/authenticate").route(web::post().to(handler::login::create_session)))
            .service(web::resource("/home").wrap(Authenticator).route(web::get().to(handler::home::index)))
            .service(web::resource("/authorize").wrap(Authenticator).route(web::get().to(handler::authorization::authorize)))
    }).bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
