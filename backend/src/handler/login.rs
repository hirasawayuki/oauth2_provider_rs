use actix_identity::Identity;
use actix_web::{HttpResponse, web, HttpRequest, HttpMessage, http::header };
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

pub async fn create(params: web::Form<LoginParams>, request: HttpRequest) -> Result<HttpResponse, HandlerError> {
    let repo = MySqlUserRepository::new().await?;
    let service = LoginService::new(Box::new(repo));
    let result = service.verify_credentials(&params.email, &params.password).await?;
    if result {
        Identity::login(&request.extensions(), "User1".into());
        return Ok(HttpResponse::Found().append_header((header::LOCATION, "/home")).finish());
    }

    Ok(HttpResponse::Ok().content_type("text/html").body("login failure"))
}

pub async fn delete(id: Identity) -> Result<HttpResponse, HandlerError> {
    id.logout();
    Ok(HttpResponse::Ok().content_type("text/html").body("logout"))
}
