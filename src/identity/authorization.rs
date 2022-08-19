use std::future::{ready, Future, Ready};
use std::pin::Pin;

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage};

use super::{AuthTokenContext, AuthenticattionInfoContext, Identity};

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_token = {
            let extensions = &req.extensions();
            let context = extensions.get::<AuthTokenContext>();
            context.map(|ctx| ctx.token.clone())
        };

        if auth_token.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "You are not authenticated",
                ))
            });
        }

        let identity = req.app_data::<Data<Identity>>();

        if identity.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorInternalServerError(
                    "Not found identity service in application context",
                ))
            });
        }

        let auth_info = identity.unwrap().authorization_info(&auth_token.unwrap());
        match auth_info {
            Ok(auth_info) => {
                req.extensions_mut()
                    .insert(AuthenticattionInfoContext::new(auth_info.clone()));

                let fut = self.service.call(req);

                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
            Err(err) => {
                return Box::pin(async { Err(err) });
            }
        };
    }
}

#[derive(Clone)]
pub struct Authorization {}

impl Authorization {
    pub fn enable() -> Self {
        Self {}
    }
}

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}
