use deadpool_postgres::Client;

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
