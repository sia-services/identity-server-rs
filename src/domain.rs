use serde::Serialize;
use tokio_postgres::Row;

#[derive(Debug, Serialize)]
pub struct User {
    pub personnel_nr: i16,
    #[serde(skip_serializing)]
    pub salt: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub password_expiration_date: chrono::Date<chrono::Utc>,
    pub username: String,
    #[serde(skip_serializing)]
    pub account_disabled: bool,
    #[serde(skip_serializing)]
    pub date_dismiss: Option<chrono::Date<chrono::Utc>>,
    pub telefon: Option<String>,
    pub email: Option<String>,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            personnel_nr: row.get(0),
            salt: row.get(1),
            password: row.get(2),
            password_expiration_date: row.get(3),
            username: row.get(4),
            account_disabled: row.get(5),
            date_dismiss: row.get(6),
            telefon: row.get(7),
            email: row.get(8),
        }
    }
}
