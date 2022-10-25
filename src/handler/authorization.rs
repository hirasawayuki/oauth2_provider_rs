use actix_identity::Identity;
use actix_web::{HttpResponse, web, http::header };
use anyhow::Context;
use chrono::{NaiveDateTime, Utc, DateTime, Duration};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use url::Url;

use crate::{repository::{oauth_client, authorize_code}, utils::gen_random_string::gen_random_string};

use super::error::HandlerError;


#[derive(Serialize, Deserialize)]
pub struct AuthorizeQuery {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
    state: String,
}

pub async fn authorize(
    ident: Identity,
    query: web::Query<AuthorizeQuery>,
    connection_pool: web::Data<MySqlPool>
) -> Result<HttpResponse, HandlerError> {
    let  oauth_client = oauth_client::find_by_client_id(&query.client_id, &connection_pool).await?;
    if oauth_client.redirect_uri != query.redirect_uri {
        return Ok(HttpResponse::BadRequest().content_type("text/html").body("invalid request"));
    }

    if oauth_client.scope != query.scope {
        return Ok(HttpResponse::BadRequest().content_type("text/html").body("invalid request"));
    }

    let code = gen_random_string(32);
    let user_id = ident.id()?;
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    let expires_at = now + Duration::minutes(10);
    authorize_code::create(&code, &user_id, &oauth_client.id.to_string(), expires_at, &connection_pool).await?;
    let params = vec![("code", &code), ("state", &query.state)];
    let url = Url::parse_with_params(&query.redirect_uri, params).context("failed to callback url")?;
    return Ok(HttpResponse::Found().append_header((header::LOCATION, url.to_string())).finish());
}

// pub async fn get_token() {
// check code
// generate access_token and refresh_token
// response access_token and refresh_tokena and expires_at
// }
