use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::OnceCell;

pub async fn init_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@0.0.0.0:5432/eszett")
        .await
        .expect("Failed to open database")
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_pool() -> &'static PgPool {
    POOL.get_or_init(|| async {
        init_pool().await
    }).await
}