
use sqlx::{Executor, Postgres};
use anyhow::Result;
use crate::language::id::LanguageId;
use super::model::{Term, TermId};

pub async fn insert_term<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    language_id: impl Into<LanguageId>,
    term: impl Into<Term>,
) -> Result<TermId> {
    let rec = sqlx::query_as!(TermId, r#"
        INSERT INTO terms (language_id, term)
        VALUES ( $1, $2 )
        RETURNING id
    "#,
        language_id.into().id,
        term.into().term,
    ).fetch_one(exec).await?;
    Ok(rec)
}

pub async fn upsert_term<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    language_id: impl Into<LanguageId>,
    term: impl Into<Term>,
) -> Result<TermId> {
    let rec = sqlx::query_as!(TermId, r#"
        INSERT INTO terms (language_id, term)
        VALUES ( $1, $2 )
        ON CONFLICT (language_id, term)
        DO UPDATE SET term = EXCLUDED.term
        RETURNING id
    "#,
        language_id.into().id,
        term.into().term,
    ).fetch_one(exec).await?;

    Ok(rec)
}

pub async fn filter_terms<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    language_id: impl Into<LanguageId>,
) -> Result<Vec<Term>> {
    let rec = sqlx::query_as!(Term, r#"
        SELECT term FROM terms
        WHERE language_id = $1 
    "#,
        language_id.into().id
    ).fetch_all(exec).await?;
    Ok(rec)
}

pub async fn find_term<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    language_id: impl Into<LanguageId>,
    term: impl Into<Term>,
) -> Result<TermId> {
    let rec = sqlx::query_as!(TermId, r#"
        SELECT id FROM terms
        WHERE language_id = $1
        AND term = $2
    "#,
        language_id.into().id,
        term.into().term,
    ).fetch_one(exec).await?;
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