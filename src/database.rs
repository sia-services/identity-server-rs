use deadpool_postgres::Client;
use tokio_postgres::GenericClient;

use crate::{domain, errors::DatabaseError};

pub async fn count_of_roles(client: &Client) -> Result<i64, DatabaseError> {
    let stmt = client
        .prepare("SELECT COUNT(*) FROM security.roles")
        .await
        .unwrap();
    let result = client.query_one(&stmt, &[]).await?;
    let value: i64 = result.get(0);
    Ok(value)
}

pub async fn find_user_by_name(
    client: &Client,
    personnel_nr: i16,
) -> Result<Option<domain::User>, DatabaseError> {
    let stmt = client
        .prepare(
            "SELECT personnel_nr, salt, password, password_expiration_date, \
            username, account_disabled, date_dismiss, telefon, email \
        FROM security.users \
        WHERE personnel_nr = $1",
        )
        .await
        .unwrap();

    let result = client.query_opt(&stmt, &[&personnel_nr]).await?;

    let user = result.map(|r| r.into());
    Ok(user)
}

pub async fn load_user_roles(
    client: &Client,
    personnel_nr: i16,
) -> Result<Vec<domain::UserRole>, DatabaseError> {
    let stmt = client
        .prepare(
            "SELECT role_id, role_name, role_group_id \
        FROM security.v_user_roles WHERE personnel_nr = $1",
        )
        .await
        .unwrap();

    let result = client.query(&stmt, &[&personnel_nr]).await?;

    let roles = result.map(|r| r.into());
    Ok(roles)
}
