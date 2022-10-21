use actix_session::Session;
use actix_web::{HttpResponse, web, http::header};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};

use crate::{repository::user::MySqlUserRepository, service::login::LoginService};
use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/login.html")]
struct LoginTemplate{}

pub async fn new() -> Result<HttpResponse, HandlerError> {
    let html = LoginTemplate{};
    let res_body = html.render().context("failed to render template")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(res_body))
}

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

pub async fn create(params: web::Form<LoginParams>, session: Session) -> Result<HttpResponse, HandlerError> {
    let repo = MySqlUserRepository::new().await?;
    let service = LoginService::new(Box::new(repo));
    let result = service.verify_credentials(&params.email, &params.password).await?;
    if result {
        match session.insert("ident", &params.email) {
            Ok(_) => return Ok(HttpResponse::Found().append_header((header::LOCATION, "/home")).finish()),
            Err(_) => return Ok(HttpResponse::Ok().content_type("text/html").body("login failure")),
        }
    }

    Ok(HttpResponse::Ok().content_type("text/html").body("login failure"))
}
