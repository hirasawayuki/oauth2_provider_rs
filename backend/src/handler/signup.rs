use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};

use crate::{service::signup::SignupService, repository::user::MySqlUserRepository};
use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/signup.html")]
struct SignupTemplate{}

pub async fn new() -> Result<HttpResponse, HandlerError> {
    let html = SignupTemplate{};
    let res_body = html.render().context("faile to render template")?;

    Ok(HttpResponse::Ok().content_type("text/html").body(res_body))
}

#[derive(Serialize, Deserialize)]
pub struct SignupParams {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

pub async fn create(
    params: web::Form<SignupParams>,
) -> Result<HttpResponse, HandlerError> {
    if params.password != params.password_confirmation {
        return Ok(HttpResponse::BadRequest().content_type("text/html").body("Password and confirmation password do not match."));
    }

    let repo = MySqlUserRepository::new().await?;
    let service = SignupService::new(Box::new(repo));
    service.register_user(&params.name, &params.email, &params.password).await?;

    return Ok(HttpResponse::Ok().content_type("text/html").body("sinup successful!"));
}
