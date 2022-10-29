use actix_web::{HttpResponse, web, HttpRequest, http::header::HeaderMap};
use base64::decode;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};
use serde::{Serialize, Deserialize};
use sqlx::{MySqlPool, Transaction, MySql};

use crate::{utils::gen_random_string::gen_random_string, models::{access_token, refresh_token, oauth_client, authorization_code}, db::establish_connection};

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
    body: web::Json<RequstBody>,
    req: HttpRequest,
) -> Result<HttpResponse, JsonError> {
    let connection_pool = match establish_connection().await {
        Ok(pool) => pool,
        Err(_) => return Err(JsonError::InternalServerError)
    };

    let headers = req.headers();
    let (client_id, client_secret) = match parse_client_credentials(headers) {
        Ok((id, secret)) => (id, secret),
        Err(_) => return Err(JsonError::BadRequest(String::from("invalid Authorization header.")))
    };

    let oauth_client = match oauth_client::find_by_client_id(&client_id, &connection_pool).await {
        Ok(oauth_client) => oauth_client,
        Err(_) => {
            return Err(JsonError::BadRequest(String::from("invalid client.")))
        }
    };

    if client_secret != &*oauth_client.client_secret {
        return Err(JsonError::BadRequest(String::from("invalid client.")));
    }

    match &*body.grant_type {
        "authorization_code" => {
            let code = match &body.code {
                Some(code) => code,
                None => return Err(JsonError::BadRequest(String::from("authorization code is required.")))
            };

            let auth_code = match authorization_code::find_by_code(&*code, &connection_pool).await {
                Ok(authorization_code) => authorization_code,
                Err(_) => {
                    return Err(JsonError::BadRequest(String::from("invalid authorization code.")));
                }
            };

            let expires_at = auth_code.expires_at;
            let utc: DateTime<Utc> = Utc::now();
            let now: NaiveDateTime = utc.naive_local();
            if (now - expires_at).num_milliseconds() > 0 {
                return Err(JsonError::BadRequest(String::from("authorization_code is expired.")))
            }

            let mut tx = match connection_pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return Err(JsonError::InternalServerError)
            };
            let (access_token, refresh_token, expires_at) = match generate_token(auth_code.user_id, &client_id, "all", &mut tx).await {
                Ok((access_token, refresh_token, expires_at)) => (access_token, refresh_token, expires_at),
                Err(_) => {
                    tx.rollback().await.unwrap_or(());
                    return Err(JsonError::InternalServerError)
                }
            };

            if let Err(_) = authorization_code::delete(&auth_code.code, &connection_pool).await {
                 tx.rollback().await.unwrap_or(());
                 return Err(JsonError::InternalServerError);
            };
            tx.commit().await.unwrap_or(());

            return Ok(HttpResponse::Ok().json(ResponseBody { access_token, refresh_token, expires_at }))
        },
        "refresh_token" => {
            let refresh_token = match &body.refresh_token {
                Some(refresh_token) => refresh_token,
                None => return Err(JsonError::BadRequest(String::from("refresh_token parameter is required."))),
            };

            let refresh_token = match refresh_token::find_by_refresh_token::<&MySqlPool>(refresh_token, &connection_pool).await {
                Ok(token) => token,
                Err(_) => return Err(JsonError::BadRequest(String::from("refresh_token parameter is required."))),
            };

            let access_token = match access_token::find_by_token::<&MySqlPool>(&refresh_token.access_token, &connection_pool).await {
                Ok(token) => token,
                Err(_) => return Err(JsonError::BadRequest(String::from("refresh_token parameter is required."))),
            };

            let mut tx = match connection_pool.begin().await {
                Ok(tx) => tx,
                Err(_) => return Err(JsonError::InternalServerError)
            };

            if let Err(_) = delete_old_token(&refresh_token.token, &mut tx).await {
                tx.rollback().await.unwrap_or(());
                return Err(JsonError::InternalServerError);
            };

            let (access_token, refresh_token, expires_at) = match generate_token(access_token.user_id, &client_id, "all", &mut tx).await {
                Ok((access_token, refresh_token, expires_at)) => (access_token, refresh_token, expires_at),
                Err(_) => {
                    tx.rollback().await.unwrap_or(());
                    return Err(JsonError::InternalServerError)
                }
            };
            tx.commit().await.unwrap_or(());

            return Ok(HttpResponse::Ok().json(ResponseBody { access_token, refresh_token, expires_at }))
        },
        _ => {
            return Err(JsonError::BadRequest(String::from("invalid grant_type.")));
        }
    }
}

async fn generate_token(user_id: u32, client_id: &str, scope: &str, tx: &mut Transaction<'static, MySql>) -> anyhow::Result<(String, String, NaiveDateTime)> {
    let access_token = gen_random_string(63);
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    let expires_at = now + Duration::minutes(30);
    if let Err(_) = access_token::create::<&mut Transaction<MySql>>(&access_token, &user_id.to_string(), client_id, scope, expires_at, tx).await {
         return Err(anyhow::anyhow!("failed to create access_token"));
    }

    let refresh_token = gen_random_string(63);
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    if let Err(_) = refresh_token::create::<&mut Transaction<MySql>>(&refresh_token, &access_token, now + Duration::days(30), tx).await {
         return Err(anyhow::anyhow!("failed to create refresh_token"));
    }

    anyhow::Ok((access_token, refresh_token, expires_at))
}

fn parse_client_credentials(headers: &HeaderMap) -> anyhow::Result<(String, String)> {
    let header_value = match headers.get("Authorization") {
        Some(value) => value,
        None => return  Err(anyhow::anyhow!("Authorization header not exists"))
    };
    let authorization_header = header_value.to_str()?;
    let encoded: Vec<&str> = authorization_header.split(" ").collect();
    let decoded_u8 = decode(encoded[1])?;
    let decoded = decoded_u8.iter().map(|&c| c as char).collect::<String>();
    let cred: Vec::<&str> = decoded.split(":").collect();
    if cred.len() != 2 {
        return  Err(anyhow::anyhow!("invalid Authorization header"))
    }

    anyhow::Ok((String::from(cred[0]), String::from(cred[1])))
}

async fn delete_old_token(refresh_token: &str, tx: &mut Transaction<'static, MySql>) -> anyhow::Result<()> {
    let refresh_token = refresh_token::find_by_refresh_token::<&mut Transaction<MySql>>(refresh_token, tx).await?;
    if let Err(e) = refresh_token::delete::<&mut Transaction<MySql>>(&refresh_token.token, tx).await {
        return Err(e);
    }
    if let Err(e) = access_token::delete::<&mut Transaction<MySql>>(&refresh_token.access_token, tx).await {
        return Err(e);
    }

    return anyhow::Ok(());
}
