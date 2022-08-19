use crate::database::count_of_roles;
use crate::errors::DatabaseError;

use actix_web::{get, HttpResponse, Result};
use deadpool_postgres::Pool;

#[get("/")]
pub async fn hello(db_pool: Data<Pool>) -> Result<HttpResponse> {
    let client = db_pool.get().await.map_err(DatabaseError::PoolError)?;

    let cnt = count_of_roles(&client).await?;
    let response = format!("Hello world!; count of roles: {:}", cnt);

    Ok(HttpResponse::Ok().body(response))
}
