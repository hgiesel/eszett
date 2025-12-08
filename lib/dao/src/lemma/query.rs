use sqlx::{Executor, Postgres};
use anyhow::Result;
use crate::term::model::TermId;
use super::model::{LemmaId};

pub async fn upsert_lemma<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    term_id: impl Into<TermId>,
) -> Result<LemmaId> {
    let rec = sqlx::query_as!(LemmaId, r#"
        INSERT INTO lemmas (term_id) VALUES ( $1 )
        ON CONFLICT(term_id) DO UPDATE
        SET term_id = EXCLUDED.term_id
        RETURNING id;
    "#,
        term_id.into().id,
    ).fetch_one(exec).await?;
    Ok(rec)
}

pub async fn find_lemma<'a>(
    exec: impl Executor<'a, Database = Postgres>,
    term_id: impl Into<TermId>,
) -> Result<Option<LemmaId>> {
    let rec = sqlx::query_as!(LemmaId, r#"
        SELECT id FROM lemmas
        WHERE term_id = $1;
    "#,
        term_id.into().id
    ).fetch_optional(exec).await?;
    Ok(rec)
}