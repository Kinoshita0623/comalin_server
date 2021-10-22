use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use actix_web::dev::{ResponseBody, Service, Transform};
use futures::future::{ok, Ready};
use futures::Future;
use actix_web::http::{HeaderName, HeaderValue};
use actix_web::http::HeaderMap;
use actix_web::http::Method;
use actix_web::http::StatusCode;
use actix_web::http::uri;
use actix_web::http::Version;
use actix_web::web::Data;
use crate::app_module::AppModule;
use crate::app_module;
use crate::user::module;

pub struct TokenAuth;


impl<S, B> Transform<S> for TokenAuth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ok(TokenAuthMiddleware { service: service } );
    }
}

pub struct TokenAuthMiddleware<S> {
    service: S
}



impl<S, B> Service for TokenAuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        return self.service.poll_ready(ctx);
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {

        let data = match req.app_data::<Data<AppModule>>() {
            Some(module) => module,
            None => {
                return Box::pin(async move {
                    Ok(
                        req.into_response(HttpResponse::InternalServerError().finish().into_body())
                    )
                });
            }
        };

        let app_module: &AppModule = data.get_ref();

        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                    let token = auth_str[6..auth_str.len()].trim();
                    if let Ok(b) = app_module.user_module().user_service().check_token(token) {
                        if b {
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                return Ok(res);
                            });
                        }

                    }
                }
            }
        }
        Box::pin(async move {
            Ok(
                req.into_response(HttpResponse::Unauthorized().finish().into_body())
            )
        })

    }


}