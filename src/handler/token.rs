use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use serde::{Serialize, Deserialize};

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

pub async fn get_token(body: web::Json<RequstBody>) -> Result<HttpResponse, JsonError> {
    // check Authorization header (client_id client_secret)
    // return access_token expires_at refresh_token, token_type
    // check refresh token
    // check Authorization header (client_id client_secret)

    match &*body.grant_type {
        "authorization_code" => {
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            let expires_at = now + Duration::minutes(10);
            return Ok(HttpResponse::Ok().json(ResponseBody {
                access_token: String::from("hoge"),
                refresh_token: String::from("hoge"),
                expires_at,
            }))
        },
        "refresh_token" => {
            return Err(JsonError::BadRequest);
        },
        _ => {
            return Err(JsonError::BadRequest);
        }
    }
}
