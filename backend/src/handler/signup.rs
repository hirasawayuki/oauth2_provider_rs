use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::{ repository::user, utils::hash_password::hash_password};
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
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, HandlerError> {
    if params.password != params.password_confirmation {
        return Ok(HttpResponse::BadRequest().content_type("text/html").body("Password and confirmation password do not match."));
    }

    let password_hash = hash_password(&params.password)?;
    user::create(&params.name, &params.email, &password_hash, &pool).await?;

    return Ok(HttpResponse::Ok().content_type("text/html").body("sinup successful!"));
}
