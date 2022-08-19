use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use deadpool_postgres::PoolError;
use derive_more::{Display, Error};
use tokio_postgres::error::Error as PGError;

#[derive(Display, Debug, Error)]
pub enum DatabaseError {
    PGError(PGError),
    PoolError(PoolError),
}

impl std::convert::From<tokio_postgres::Error> for DatabaseError {
    fn from(error: PGError) -> Self {
        DatabaseError::PGError(error)
    }
}

impl error::ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            DatabaseError::PoolError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}
