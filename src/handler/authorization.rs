use actix_identity::Identity;
use actix_web::{HttpResponse, web, http::header };
use chrono::{NaiveDateTime, Utc, DateTime, Duration};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use url::Url;

use crate::{models::{oauth_client, authorization_code }, utils::gen_random_string::gen_random_string};

use super::error::JsonError;

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
) -> Result<HttpResponse, JsonError> {
    let oauth_client = match oauth_client::find_by_client_id(&query.client_id, &connection_pool).await {
        Ok(oauth_client) => { oauth_client },
        Err(_) => {
            return Err(JsonError::BadRequest(String::from("invalid request params.")));
        }
    };

    if oauth_client.redirect_uri != query.redirect_uri {
         return Err(JsonError::BadRequest(String::from("invalid redirect_uri.")));
    }

    if oauth_client.scope != query.scope {
         return Err(JsonError::BadRequest(String::from("invalid scope")));
    }

    let user_id = match ident.id() {
        Ok(user_id) => user_id,
        Err(_) => { return Err(JsonError::InternalServerError); }
    };

    let code = gen_random_string(63);
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    let expires_at = now + Duration::minutes(10);
    if let Err(e) = authorization_code::create(&code, &user_id, &oauth_client.client_id, expires_at, &connection_pool).await {
        println!("{}", e);
         return Err(JsonError::InternalServerError);
    }

    let params = vec![("code", &code), ("state", &query.state)];
    match Url::parse_with_params(&query.redirect_uri, params) {
        Ok(callback_url) => Ok(HttpResponse::Found().append_header((header::LOCATION, callback_url.to_string())).finish()),
        Err(_) => Err(JsonError::InternalServerError)
    }
}
