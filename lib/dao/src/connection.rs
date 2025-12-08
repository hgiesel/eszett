use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn init_pool() -> PgPool {

    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@0.0.0.0:5432/eszett").await.expect("Failed to open database")
}