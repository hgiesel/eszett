use sqlx::{PgPool, Acquire, Postgres, PgConnection};

pub trait Connector<'c> : Acquire<'c, Database = Postgres> + Sized {
    fn get_connection(self) -> impl Future<Output = Result<Self::Connection, sqlx::error::Error>>;
}

impl Connector<'_> for &PgPool {
    async fn get_connection(self) -> Result<Self::Connection, sqlx::error::Error> {
        self.acquire().await
    }
}

impl <'c> Connector<'c> for &'c mut PgConnection {
    async fn get_connection(self) -> Result<Self::Connection, sqlx::error::Error> {
        Ok(&mut *self)
    }
}
