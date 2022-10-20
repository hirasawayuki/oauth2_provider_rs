use actix_web::{HttpResponse, Error};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};

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

pub async fn create() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().content_type("text/html").body(""))
}
