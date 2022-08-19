use serde::Serialize;
use tokio_postgres::Row;

#[derive(Serialize)]
pub struct User {
    pub personnel_nr: i16,
    #[serde(skip_serializing)]
    pub salt: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub password_expiration_date: chrono::NaiveDate,
    pub username: String,
    #[serde(skip_serializing)]
    pub account_disabled: bool,
    #[serde(skip_serializing)]
    pub date_dismiss: Option<chrono::NaiveDate>,
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

#[derive(Serialize)]
pub struct UserRole {
    #[serde(skip_serializing)]
    pub role_id: i16,
    pub role_name: String,
}

impl From<Row> for UserRole {
    fn from(row: Row) -> Self {
        Self {
            role_id: row.get(0),
            role_name: row.get(1),
        }
    }
}

#[derive(Serialize)]
pub struct UserResource {
    #[serde(skip_serializing)]
    pub resource_id: i16,
    pub resource_name: String,
    pub with_write_or_execution: bool,
}

impl From<Row> for UserResource {
    fn from(row: Row) -> Self {
        Self {
            resource_id: row.get(0),
            resource_name: row.get(1),
            with_write_or_execution: row.get(2),
        }
    }
}
