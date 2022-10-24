use actix_identity::Identity;
use actix_web::{HttpResponse, web, HttpRequest, HttpMessage, http::header };
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::{utils::hash_password::verify_password, repository::user};
use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/login.html")]
struct LoginTemplate{}

pub async fn new_session() -> Result<HttpResponse, HandlerError> {
    let html = LoginTemplate{};
    let res_body = html.render().context("failed to render template")?;

    Ok(HttpResponse::Ok().content_type("text/html").body(res_body))
}

#[derive(Serialize, Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

pub async fn create_session(
    params: web::Form<LoginParams>,
    connection_pool: web::Data<MySqlPool>,
    request: HttpRequest
) -> Result<HttpResponse, HandlerError> {
    let user = user::find_by_email(&params.email, &connection_pool).await?;
    if verify_password(&user.password, &params.password)? {
        Identity::login(&request.extensions(), String::from(&params.email));
        return Ok(HttpResponse::Found().append_header((header::LOCATION, "/home")).finish());
    }

    Ok(HttpResponse::BadRequest().content_type("text/html").body("login failure"))
}

pub async fn delete_session(id: Identity) -> Result<HttpResponse, HandlerError> {
    id.logout();
    Ok(HttpResponse::Ok().content_type("text/html").body("logout"))
}
