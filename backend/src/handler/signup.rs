use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use super::error::HandlerError;
use super::utils::hash_password;

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
    create_user(&params.name, &params.email, &params.password, &pool).await?;

    Ok(HttpResponse::Ok().content_type("text/html").body("sinup successful!"))
}


#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

async fn create_user(name: &str, email: &str, password: &str, pool: &MySqlPool) -> anyhow::Result<()> {
    let password_hash = hash_password(password)?;

    sqlx::query(
        r#"
INSERT INTO users (name, email, password) VALUES (?, ?, ?);
        "#)
        .bind(name)
        .bind(email)
        .bind(String::from(password_hash))
        .execute(pool)
        .await?;

    Ok(())
}

