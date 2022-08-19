use deadpool_postgres::Client;

use crate::errors::DatabaseError;

pub async fn count_of_roles(client: &Client) -> Result<i64, DatabaseError> {
    let stmt = client
        .prepare("SELECT COUNT(*) FROM security.roles")
        .await
        .unwrap();
    let result = client.query_one(&stmt, &[]).await?;
    let value: i64 = result.get(0);
    Ok(value)
}
