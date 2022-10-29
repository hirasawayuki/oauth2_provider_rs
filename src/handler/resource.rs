use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use super::error::JsonError;

#[derive(Serialize, Deserialize)]
pub struct ResponseBody {
    message: String,
}

pub async fn index() -> Result<HttpResponse, JsonError> {
    Ok(HttpResponse::Ok().json(ResponseBody {
        message: String::from("Successfully verified access token")
    }))
}
