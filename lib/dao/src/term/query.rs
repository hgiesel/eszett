
use sqlx::{Executor, Postgres};
use anyhow::Result;
use super::model::TermId;

pub async fn insert_term<'a>(exec: impl Executor<'a, Database = Postgres>, language_id: i32, term: &str) -> Result<TermId> {
    let rec = sqlx::query!(r#"
        INSERT INTO terms (language_id, term)
        VALUES ( $1, $2 )
        RETURNING id
    "#, language_id, term).fetch_one(exec).await?;

    Ok(rec.id.into())
}

pub async fn upsert_term<'a>(exec: impl Executor<'a, Database = Postgres>, language_id: i32, term: &str) -> Result<TermId> {
    let rec = sqlx::query!(r#"
        INSERT INTO terms (language_id, term)
        VALUES ( $1, $2 )
        ON CONFLICT (language_id, term)
        DO UPDATE SET term = EXCLUDED.term
        RETURNING id
    "#, language_id, term).fetch_one(exec).await?;

    Ok(rec.id.into())
}

pub async fn filter_terms<'a>(pool: impl Executor<'a, Database = Postgres>, language_id: i32) -> Result<Vec<String>> {
    let rec = sqlx::query!(r#"
        SELECT term FROM terms
        WHERE language_id = $1 
    "#, language_id).fetch_all(pool).await?;
    Ok(rec.into_iter().map(|x| x.term).collect())
}

pub async fn find_term<'a>(exec: impl Executor<'a, Database = Postgres>, language_id: i32, term: &str) -> Result<TermId> {
    let rec = sqlx::query_as!(TermId, r#"
        SELECT id FROM terms
        WHERE language_id = $1
        AND term = $2
    "#, language_id, term).fetch_one(exec).await?;
    Ok(rec)
}

#[tokio::test]
async fn finds_a_word() {
    let pool = crate::connection::init_pool().await;
    let mut tr = pool.begin().await.unwrap();
    let x = find_term(tr.as_mut(), 1, "eat").await.unwrap();
    tr.commit().await.unwrap();
    assert_eq!(x, 452115.into());
}