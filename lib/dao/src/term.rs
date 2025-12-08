use super::connection::get_pool;

pub async fn query() -> Result<i64, sqlx::Error> {
    let pool = get_pool().await;

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL/MariaDB)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(pool).await?;

    Ok(row.0)
}

#[tokio::test]
async fn foo() {
    let x = query().await.unwrap();
    assert_eq!(x, 150);
}