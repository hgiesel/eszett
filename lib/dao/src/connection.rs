use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::OnceCell;

async fn init_pool() -> PgPool {

    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/postgres").await.expect("Failed to open database")
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_pool() -> &'static PgPool {
    POOL.get_or_init(init_pool).await
}
