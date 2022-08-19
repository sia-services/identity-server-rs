use deadpool_postgres::Client;

use crate::{errors::DatabaseError, domain};

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
    username: &str,
) -> Result<Option<domain::User>, IdentityServerError> {
    let stmt = client
        .prepare(
            "SELECT personnel_nr, salt, password, username, email \
        FROM security.users \
        WHERE personnel_nr = $1",
        )
        .await
        .unwrap();

    log::info!("authentication statement prepared");

    let personnel_nr: i16 = FromStr::from_str(username).map_err(|_| {
        IdentityServerError::validationError("username must be personnel nr: number")
    })?;

    let result = client.query_opt(&stmt, &[&personnel_nr]).await?;

    log::info!("authentication result retrieved");

    let user = result.map(|r| r.into());
    Ok(user)
}
