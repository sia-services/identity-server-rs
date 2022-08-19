use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;

use super::AuthTokenContext;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Clone)]
pub struct AuthTokenMiddlewareFactory;

impl AuthTokenMiddlewareFactory {
    pub fn new() -> Self {
        Self {}
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthTokenMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthTokenMiddleware { service }))
    }
}

pub struct AuthTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match self.construct_context(&req) {
            Ok(_) => {
                let fut = self.service.call(req);

                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(err) => Box::pin(async { Err(actix_web::error::ErrorBadRequest(err)) }),
        }
    }
}

impl<S, B> AuthTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    fn construct_context(&self, req: &ServiceRequest) -> Result<(), Error> {
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            return Ok(());
        }

        let token = auth_header
            .unwrap()
            .to_str()
            .map_err(|e| actix_web::error::ErrorBadRequest(e))?;
        let mut segments = token.split(" ");

        let auth_type = segments.next().unwrap();
        let auth_token = segments.next();

        if auth_type != "Token" || auth_token.is_none() {
            return Err(actix_web::error::ErrorBadRequest(
                "Invalid authorization info",
            ));
        }

        let token = auth_token.unwrap();
        req.extensions_mut()
            .insert(AuthTokenContext::new(token.to_owned()));
        Ok(())
    }
}
