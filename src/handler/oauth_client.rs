use actix_identity::Identity;
use actix_web::{HttpResponse, web};
use askama::Template;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::{models::oauth_client, utils::gen_random_string::gen_random_string, entity::oauth_client::OAuthClient};

use super::error::HtmlError;

#[derive(Template)]
#[template(path="../templates/oauth_client/new.html")]
struct ClientFormTemplate{}

pub async fn new() -> Result<HttpResponse, HtmlError> {
    let html = ClientFormTemplate{};
    match html.render() {
        Ok(body) => Ok(HttpResponse::Ok().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClientParams {
    name: String,
    redirect_uri: String,
}

#[derive(Template)]
#[template(path="../templates/oauth_client/index.html")]
struct IndexTemplate {
    oauth_clients: Vec<OAuthClient>
}

pub async fn index(ident: Identity, connection_pool: web::Data<MySqlPool>) -> Result<HttpResponse, HtmlError> {
    let user_id = match ident.id() {
        Ok(user_id) => user_id,
        Err(_) => { return Err(HtmlError::Status5XX); }
    };

    let oauth_clients = match oauth_client::find_by_user_id(&user_id, &connection_pool).await {
        Ok(oauth_clients) => oauth_clients,
        Err(_) => vec![]
    };

    let html = IndexTemplate { oauth_clients };
    match html.render() {
        Ok(body) => Ok(HttpResponse::BadRequest().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX)
    }
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
) -> Result<HttpResponse, HtmlError> {
    let uuid = Uuid::new_v4();
    let client_id = uuid.to_string();
    println!("{}", client_id);
    let client_secret = gen_random_string(63);
    let user_id = match id.id() {
        Ok(user_id) => user_id,
        Err(_) => { return Err(HtmlError::Status5XX); }
    };

    if let Err(_) = oauth_client::create(&client_id, &client_secret, &params.name, &user_id, &params.redirect_uri, "all", &connection_pool).await {
        return Err(HtmlError::Status5XX);
    }

    let html = ClientTemplate {
        name: params.name.clone(),
        redirect_uri: params.redirect_uri.clone(),
        client_id,
        client_secret,
    };

    match html.render() {
        Ok(body) => Ok(HttpResponse::BadRequest().content_type("text/html").body(body)),
        Err(_) => Err(HtmlError::Status5XX)
    }
}
