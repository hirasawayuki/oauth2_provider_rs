use actix_identity::Identity;
use actix_session::SessionExt;
use actix_web::{HttpResponse, web, HttpRequest, HttpMessage, http::header };
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::{utils::hash_password::verify_password, models::user};
use super::error::HtmlError;

#[derive(Template)]
#[template(path="../templates/login.html")]
struct LoginTemplate{}

pub async fn new_session() -> Result<HttpResponse, HtmlError> {
    let html = LoginTemplate{};
    match html.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX)
    }
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
) -> Result<HttpResponse, HtmlError> {
    let user = match user::find_by_email(&params.email, &connection_pool).await {
        Ok(user) => user,
        Err(_) => { return Err(HtmlError::Status4XX); }
    };

    let result = match verify_password(&user.password, &params.password) {
        Ok(result) => result,
        Err(_) => { return Err(HtmlError::Status5XX); }
    };

    if !result {
        return Err(HtmlError::Status4XX);
    }

    if let Err(_) = Identity::login(&request.extensions(), user.id.to_string()) {
        return Err(HtmlError::Status5XX);
    }

    let session = request.get_session();
    match session.get::<String>("redirect_url") {
        Ok(redirect_url) => {
            session.remove("redirect_url");
            if let Some(redirect_url) = redirect_url {
                return Ok(HttpResponse::Found().append_header((header::LOCATION, redirect_url)).finish());
            } else {
                return Ok(HttpResponse::Found().append_header((header::LOCATION, "/home")).finish());
            }
        },
        Err(_) => { return Err(HtmlError::Status5XX); }
    }
}

pub async fn delete_session(id: Identity) -> Result<HttpResponse, HtmlError> {
    id.logout();
    return Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish());
}
