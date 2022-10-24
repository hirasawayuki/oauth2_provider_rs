use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use anyhow::Context;
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::{repository::oauth_client, utils::gen_random_string::gen_random_string};

use super::error::HandlerError;

#[derive(Template)]
#[template(path="../templates/oauth_client/new.html")]
struct ClientFormTemplate{}

pub async fn new() -> Result<HttpResponse, HandlerError> {
    let html = ClientFormTemplate{};
    let res_body = html.render().context("failed to render template")?;
    Ok(HttpResponse::Ok().content_type("text/html").body(res_body))
}

#[derive(Serialize, Deserialize)]
pub struct ClientParams {
    name: String,
    redirect_uri: String,
}

#[derive(Template)]
#[template(path="../templates/oauth_client/show.html")]
struct ClientTemplate{
    name: String,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

pub async fn create(
    params: web::Form<ClientParams>,
    connection_pool: web::Data<MySqlPool>,
    id: Identity
) -> Result<HttpResponse, HandlerError> {
    let uuid = Uuid::new_v4();
    let secret = gen_random_string(255);
    let user_id = id.id()?;
    oauth_client::create(&user_id, &params.name, &params.redirect_uri, "all", &uuid.to_string(), &secret, &connection_pool).await?;
    let html = ClientTemplate {
        name: params.name.clone(),
        redirect_uri: params.redirect_uri.clone(),
        client_id: uuid.to_string(),
        client_secret: secret,
    };

    let res_body = html.render().context("failed to render template")?;
    Ok(HttpResponse::BadRequest().content_type("text/html").body(res_body))
}
