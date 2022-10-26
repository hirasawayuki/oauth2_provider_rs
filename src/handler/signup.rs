use actix_web::{HttpResponse, web, http::header};
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::{ repository::user, utils::hash_password::hash_password};
use super::error::HtmlError;

#[derive(Template)]
#[template(path="../templates/signup.html")]
struct SignupTemplate{}

pub async fn new() -> Result<HttpResponse, HtmlError> {
    let html = SignupTemplate{};
    match html.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX)
    }
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
) -> Result<HttpResponse, HtmlError> {
    if params.password != params.password_confirmation {
        return Err(HtmlError::Status4XX);
    }

    let password_hash = match hash_password(&params.password) {
        Ok(password_hash) => password_hash,
        Err(_) => { return Err(HtmlError::Status5XX); }
    };

    match user::create(&params.name, &params.email, &password_hash, &pool).await {
        Ok(_) => Ok(HttpResponse::Found().append_header((header::LOCATION, "/home")).finish()),
        Err(_) => { return Err(HtmlError::Status5XX); }
    }
}
