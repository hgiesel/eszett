use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware, Extension};
use base::language::Language;
use dao::connection::get_pool;
use dao::language::dao::LanguageDao;
use dao::term::query::find_term;
use dao::term::term::Term;
use crate::language::extractor::ExtractLanguage;

async fn validate_term(
    ExtractLanguage(language): ExtractLanguage,
    Path(term): Path<Term>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let conn = get_pool().await;
    let found = find_term(conn, LanguageDao::English, term.clone()).await;

    match found {
        Ok(Some(found)) => {
            req.extensions_mut().insert(term);
            req.extensions_mut().insert(found);
            Ok(next.run(req).await)
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn term_route(router: Router) -> Router {
    Router::new()
        .nest("/{term}", router)
        .layer(middleware::from_fn(validate_term))
}
