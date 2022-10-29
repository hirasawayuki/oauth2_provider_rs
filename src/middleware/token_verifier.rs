use std::future::{Ready, ready};
use std::rc::Rc;

use actix_web::body::EitherBody;
use actix_web::dev::{self, Transform, ServiceRequest, Service, ServiceResponse};
use actix_web::error::{ErrorUnauthorized, ErrorInternalServerError};
use actix_web::{Error, HttpResponse, web};
use actix_web::http::header:: HeaderMap;
use anyhow::Context;
use chrono::{Utc, NaiveDateTime, DateTime};
use futures_util::future::LocalBoxFuture;
use sqlx::MySqlPool;

use crate::models::access_token;

pub struct TokenVerifier;

impl<S, B> Transform<S, ServiceRequest> for TokenVerifier
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = GuardResourceMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GuardResourceMiddleware{ service: Rc::new(service) }))
    }
}

pub struct GuardResourceMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for GuardResourceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            match parse_access_token(request.headers()) {
                Ok(access_token) => {
                    let connection_pool = match request.app_data::<web::Data<MySqlPool>>() {
                        Some(pool) => pool,
                        None => {
                            return Err(ErrorInternalServerError("failed to db connection"));
                        }
                    };

                    match verify_access_token(&access_token, &connection_pool).await {
                        Ok(_) => {
                            let res = svc.call(request);
                            return res.await.map(ServiceResponse::map_into_left_body)
                        },
                        Err(e) => {
                            return Err(ErrorUnauthorized(e));
                        }
                    }
                },
                Err(_) => {
                    println!("faile");
                    let (request, _pl) = request.into_parts();
                    let response = HttpResponse::Unauthorized().finish().map_into_right_body();
                    return Ok(ServiceResponse::new(request, response));
                }
            }
        })
    }
}

fn parse_access_token(headers: &HeaderMap) -> anyhow::Result<String> {
    let header_value = match headers.get("Authorization") {
        Some(value) => value,
        None => return  Err(anyhow::anyhow!("Authorization header not exists"))
    };
    let authorization_header = header_value.to_str()?;
    let bearer: Vec<&str> = authorization_header.split(" ").collect();
    if bearer.len() != 2 {
        return Err(anyhow::anyhow!("Authorization header not exists"))
    }

    anyhow::Ok(String::from(bearer[1]))
}

async fn verify_access_token(token: &str, connection_pool: &MySqlPool) -> anyhow::Result<()> {
    let access_token = access_token::find_by_token(token, connection_pool).await.context("invalid access_token")?;
    let utc: DateTime<Utc> = Utc::now();
    let now: NaiveDateTime = utc.naive_local();
    if (access_token.expires_at - now).num_milliseconds() <= 0 {
        return Err(anyhow::anyhow!("access_token is expired"))
    }

    anyhow::Ok(())
}
