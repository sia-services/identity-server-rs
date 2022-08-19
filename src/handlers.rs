use crate::database::count_of_roles;
use crate::errors::DatabaseError;

use actix_web::{get, web, HttpResponse, Result};
use deadpool_postgres::Pool;

#[get("/")]
pub async fn hello(db_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let client = db_pool.get().await.map_err(DatabaseError::PoolError)?;

    let cnt = count_of_roles(&client).await?;
    let response = format!("Hello world!; count of roles: {:}", cnt);

    Ok(HttpResponse::Ok().body(response))
}

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

#[post("/login")]
pub async fn login(
    db_pool: Data<Pool>,
    identity: Data<Identity>,
    credentials: web::Json<UsernamePasswordCredentials>,
) -> Result<impl Responder> {
    let client: Client = db_pool
        .get()
        .await
        .map_err(IdentityServerError::PoolError)?;
    let maybe_user = find_user_by_name(&client, &credentials.username).await?;

    let user = maybe_user.ok_or(actix_web::error::ErrorUnauthorized(
        "Utilizatorul cu acest nume nu este autentificat",
    ))?;

    log::info!("authenticated user: {:?}", &user);

    let response = identity.authenticate(user, &credentials.password)?;

    Ok(web::Json(response))
}

#[get("/info")]
pub async fn auth_info(
    auth_context: Option<ReqData<AuthenticattionInfoContext>>,
) -> Result<impl Responder> {
    let auth_context = auth_context.ok_or(actix_web::error::ErrorInternalServerError(
        "Authentication info context not found in application",
    ))?;

    let auth_user = auth_context.auth_info.clone();

    Ok(web::Json(auth_user))
}

#[post("/logout")]
pub async fn logout(
    identity: Data<Identity>,
    token_context: Option<ReqData<AuthTokenContext>>,
) -> Result<HttpResponse> {
    if token_context.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }
    let token = &token_context.unwrap().token;
    identity.logout(token)?;
    Ok(HttpResponse::Ok().finish())
}
