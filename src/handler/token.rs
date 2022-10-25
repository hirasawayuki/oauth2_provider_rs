// use actix_web::{HttpResponse, web};

// use super::error::HandlerError;

// #[derive(Serialize, Deserialize)]
// pub struct Body {
//     grant_type: String,
//     code: Option<String>,
//     refresh_token: Option<String>,
// }

// pub async fn get_token(body: web::Json<Body>) -> Result<HttpResponse, HandlerError> {
//     // check grant type  -> refresh_token or authorization_code
//     // check Authorization header (client_id client_secret)
//     // return access_token expires_at refresh_token, token_type
//     // check refresh token
//     // check Authorization header (client_id client_secret)
// }
