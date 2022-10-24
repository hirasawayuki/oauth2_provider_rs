use std::future::{Ready, ready};

use actix_identity::IdentityExt;
use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{self, Transform, ServiceRequest, Service, ServiceResponse};
use actix_web::{Error, HttpResponse};
use actix_web::http::header;
use futures_util::future::LocalBoxFuture;

pub struct Authenticator;

impl<S, B> Transform<S, ServiceRequest> for Authenticator
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let is_logged_in = match request.get_identity() {
            Ok(_) => true,
            Err(_) => false,
        };

        if !is_logged_in && request.path() != "/login" {
            let session = request.get_session();
            session.insert("redirect_url", request.uri().to_string());
            let (request, _pl) = request.into_parts();

            let response = HttpResponse::Found().insert_header((header::LOCATION, "/login"))
                .finish().map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(request, response))});
        }

        let res = self.service.call(request);
        Box::pin(async move {
            res.await.map(ServiceResponse::map_into_left_body)
        })
    }
}
