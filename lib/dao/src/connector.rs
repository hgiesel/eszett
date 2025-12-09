use std::result;
use sqlx::{PgPool, Acquire, Postgres, PgConnection};

pub trait Connector<'a> : Acquire<'a, Database = Postgres> + Sized {
    fn get_connection(self) -> impl Future<Output = Result<Self::Connection, sqlx::error::Error>>;
}

impl Connector<'_> for &PgPool {
    async fn get_connection(self) -> Result<Self::Connection, sqlx::error::Error> {
        self.acquire().await
    }
}

impl <'a> Connector<'a> for &'a mut PgConnection {
    async fn get_connection(self) -> Result<Self::Connection, sqlx::error::Error> {
        Ok(&mut *self)
    }
}