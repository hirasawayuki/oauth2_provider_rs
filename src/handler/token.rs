use actix_identity::Identity;
use actix_web::{HttpResponse, web, HttpRequest};
use base64::decode;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;

use crate::{utils::gen_random_string::gen_random_string, repository::{access_token, refresh_token, oauth_client, authorization_code}};

use super::error::JsonError;

#[derive(Serialize, Deserialize)]
pub struct RequstBody {
    grant_type: String,
    code: Option<String>,
    refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseBody {
    access_token: String,
    refresh_token: String,
    expires_at: NaiveDateTime,
}

pub async fn get_token(
    ident: Identity,
    body: web::Json<RequstBody>,
    req: HttpRequest,
    connection_pool: web::Data<MySqlPool>
) -> Result<HttpResponse, JsonError> {
    let headers = req.headers();
    let header_value = match headers.get("Authorization") {
        Some(authorization_header) => { authorization_header },
        None => { return Err(JsonError::BadRequest) }
    };

    let authorization_header = match header_value.to_str() {
        Ok(header) => header,
        Err(_) => { return Err(JsonError::BadRequest) }
    };

    let encoded: Vec<&str> = authorization_header.split(" ").collect();
    let decoded = match decode(encoded[1]) {
        Ok(decoded) => { decoded.iter().map(|&c| c as char).collect::<String>()},
        Err(_) => { return Err(JsonError::BadRequest) }
    };
    let cred: Vec::<&str> = decoded.split(":").collect();
    let client_id = cred[0];
    let client_secret = cred[1];
    let oauth_client = match oauth_client::find_by_client_id(client_id, &connection_pool).await {
        Ok(oauth_client) => oauth_client,
        Err(_) => { return Err(JsonError::BadRequest) }
    };

    if client_secret != &*oauth_client.client_secret {
        return Err(JsonError::BadRequest);
    }

    let user_id = match ident.id() {
        Ok(user_id) => user_id,
        Err(_) => { return Err(JsonError::InternalServerError); }
    };

    let code = match &body.code {
        Some(code) => code,
        None => { return Err(JsonError::InternalServerError); }
    };
    let authorizaion_code = match authorization_code::find_by_code(&*code, &connection_pool).await {
        Ok(authorization_code) => authorization_code,
        Err(_) => { return Err(JsonError::BadRequest); }
    };

    let expires_at = match NaiveDateTime::parse_from_str(&authorizaion_code.expires_at, "%Y-%m-%d %H:%M:%S") {
        Ok(expires_at) => expires_at,
        Err(_) => return Err(JsonError::BadRequest),
    };
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    if (now - expires_at).num_milliseconds() > 0 {
        return Err(JsonError::BadRequest)
    }

    match &*body.grant_type {
        "authorization_code" => {
            let access_token = gen_random_string(32);
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            let expires_at = now + Duration::minutes(30);
            if let Err(_) = access_token::create(&access_token, &user_id, &oauth_client.id.to_string(), "all", expires_at, &connection_pool).await {
                 return Err(JsonError::InternalServerError);
            }

            let refresh_token = gen_random_string(32);
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            let expires_at = now + Duration::days(30);
            if let Err(_) = refresh_token::create(&refresh_token, &access_token, expires_at, &connection_pool).await {
                 return Err(JsonError::InternalServerError);
            }

            return Ok(HttpResponse::Ok().json(ResponseBody {
                access_token,
                refresh_token,
                expires_at,
            }))
        },
        "refresh_token" => {
            let refresh_token = match &body.refresh_token {
                Some(refresh_token) => refresh_token,
                None => return Err(JsonError::BadRequest),
            };
            match refresh_token::find_by_refresh_token(refresh_token, &connection_pool).await {
                Ok(refresh_token) => {
                    if let Err(_) = access_token::delete(&refresh_token.access_token, &connection_pool).await {
                        return Err(JsonError::BadRequest);
                    }
                    if let Err(_) = refresh_token::delete(&refresh_token.token, &connection_pool).await {
                        return Err(JsonError::BadRequest);
                    }
                },
                Err(_) => return Err(JsonError::BadRequest),
            };

            let access_token = gen_random_string(32);
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            let expires_at = now + Duration::minutes(30);
            if let Err(_) = access_token::create(&access_token, &user_id, &oauth_client.id.to_string(), "all", expires_at, &connection_pool).await {
                 return Err(JsonError::InternalServerError);
            }

            let refresh_token = gen_random_string(32);
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            let expires_at = now + Duration::days(30);
            if let Err(_) = refresh_token::create(&refresh_token, &access_token, expires_at, &connection_pool).await {
                 return Err(JsonError::InternalServerError);
            }

            return Ok(HttpResponse::Ok().json(ResponseBody {
                access_token,
                refresh_token,
                expires_at,
            }))
        },
        _ => {
            return Err(JsonError::BadRequest);
        }
    }
}
