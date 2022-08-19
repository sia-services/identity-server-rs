use crate::database::{count_of_roles, find_user_by_name, load_user_roles};
use crate::errors::DatabaseError;
use crate::identity::{AuthTokenContext, AuthenticattionInfoContext, Authorization, Identity};

use actix_web::{get, post, web, HttpResponse, Responder, Result};
use deadpool_postgres::Pool;
use serde::Deserialize;

#[get("/")]
pub async fn hello(db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client = db_pool.get().await.map_err(DatabaseError::PoolError)?;

    let cnt = count_of_roles(&client).await?;
    let response = format!("Hello world!; count of roles: {:}", cnt);

    Ok(HttpResponse::Ok().body(response))
}

use actix_web::dev::HttpServiceFactory;

pub fn auth_scope() -> impl HttpServiceFactory {
    web::scope("/auth")
        .wrap(Authorization::enable())
        .service(auth_info)
}

#[derive(Deserialize)]
pub struct UsernamePasswordCredentials {
    username: String,
    password: String,
}

use std::str::FromStr;

#[post("/login")]
pub async fn login(
    db_pool: web::Data<Pool>,
    identity: web::Data<Identity>,
    credentials: web::Json<UsernamePasswordCredentials>,
) -> Result<impl Responder> {
    let client = db_pool.get().await.map_err(DatabaseError::PoolError)?;

    let personnel_nr: i16 = FromStr::from_str(&credentials.username)
        .map_err(|_| actix_web::error::ErrorBadRequest("username must be personnel nr: number"))?;

    let maybe_user = find_user_by_name(&client, personnel_nr).await?;

    let user = maybe_user.ok_or(actix_web::error::ErrorUnauthorized(
        "Utilizatorul cu acest nume nu este autentificat",
    ))?;

    log::info!(
        "authenticated user: {} / {}",
        &user.username,
        &user.personnel_nr
    );

    identity.verify_authentication(&user, &credentials.password)?;

    let user_roles = load_user_roles(&client, personnel_nr).await?;

    let response = identity.authenticate(user, user_roles)?;

    Ok(web::Json(response))
}

#[get("/info")]
pub async fn auth_info(
    auth_context: Option<web::ReqData<AuthenticattionInfoContext>>,
) -> Result<impl Responder> {
    let auth_context = auth_context.ok_or(actix_web::error::ErrorInternalServerError(
        "Authentication info context not found in application",
    ))?;

    let auth_user = auth_context.auth_info.clone();

    Ok(web::Json(auth_user))
}

#[post("/logout")]
pub async fn logout(
    identity: web::Data<Identity>,
    token_context: Option<web::ReqData<AuthTokenContext>>,
) -> Result<HttpResponse> {
    if token_context.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }
    let token = &token_context.unwrap().token;
    identity.logout(token)?;
    Ok(HttpResponse::Ok().finish())
}
