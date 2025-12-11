use anyhow::Result;
use crate::id::IdType;
use crate::language::dao::LanguageDao;
use super::term_id::TermId;
use super::term::Term;
use crate::connector::Connector;


pub async fn insert_term<'a>(
    conn: impl Connector<'a>,
    language_id: impl Into<LanguageDao>,
    term: impl Into<Term>,
) -> Result<TermId> {
    let conn = &mut *conn.get_connection().await?;
    let rec = sqlx::query_as!(TermId, r#"
    INSERT INTO terms (language_id, term)
    VALUES ( $1, $2 )
    RETURNING id
"#,
    language_id.into() as IdType,
    term.into().to_string(),
).fetch_one(conn).await?;
    Ok(rec)
}

pub async fn upsert_term<'a>(
    conn: impl Connector<'a>,
    language_id: impl Into<LanguageDao>,
    term: impl Into<Term>,
) -> Result<TermId> {
    let conn = &mut *conn.get_connection().await?;
    let rec = sqlx::query_as!(TermId, r#"
    INSERT INTO terms (language_id, term)
    VALUES ( $1, $2 )
    ON CONFLICT (language_id, term)
    DO UPDATE SET term = EXCLUDED.term
    RETURNING id
"#,
    language_id.into() as IdType,
    term.into().to_string(),
).fetch_one(conn).await?;
    Ok(rec)
}

pub async fn filter_terms<'a>(
    conn: impl Connector<'a>,
    language_id: impl Into<LanguageDao>,
) -> Result<Vec<Term>> {
    let conn = &mut *conn.get_connection().await?;
    let rec = sqlx::query_as!(Term, r#"
    SELECT term FROM terms
    WHERE language_id = $1
"#,
    language_id.into() as IdType,
).fetch_all(conn).await?;
    Ok(rec)
}

pub async fn find_term<'a>(
    conn: impl Connector<'a>,
    language_id: impl Into<LanguageDao>,
    term: impl Into<Term>,
) -> Result<Option<TermId>> {
    let conn = &mut *conn.get_connection().await?;
    let rec = sqlx::query_as!(TermId, r#"
    SELECT id FROM terms
    WHERE language_id = $1
    AND term = $2
"#,
    language_id.into() as IdType,
    term.into().to_string(),
).fetch_optional(conn).await?;
    Ok(rec)
}

#[tokio::test]
async fn finds_a_word() {
    let pool = crate::connection::get_pool().await;
    let term_id = find_term(pool, LanguageDao::English, "cat").await.unwrap();
    assert!(term_id.is_some());
}
