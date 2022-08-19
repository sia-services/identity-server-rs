use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug,Serialize)]
pub struct User {
    pub personnel_nr: i16,
    pub salt: String,
    pub password: String,
    pub username: String,
    pub email: Option<String>,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            personnel_nr: row.get(0),
            salt: row.get(1),
            password: row.get(2),
            username: row.get(3),
            email: row.get(4),
        }
    }
}
