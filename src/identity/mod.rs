mod auth_token;
mod authorization;
mod service;

#[derive(Clone)]
pub struct AuthTokenContext {
    pub token: Rc<String>,
}

#[derive(Clone)]
pub struct AuthenticattionInfoContext {
    pub auth_info: Arc<AuthenticatedUser>,
}

impl AuthTokenContext {
    pub fn new(token: String) -> Self {
        Self {
            token: Rc::new(token),
        }
    }
}

impl AuthenticattionInfoContext {
    pub fn new(auth_info: Arc<AuthenticatedUser>) -> Self {
        Self { auth_info }
    }
}

use std::{rc::Rc, sync::Arc};

pub use auth_token::AuthTokenMiddlewareFactory;
pub use authorization::Authorization;
pub use service::Identity;

use self::service::AuthenticatedUser;
