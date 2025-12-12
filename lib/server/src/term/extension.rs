use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware};
use dao::connection::get_pool;
use dao::language::dao::LanguageDao;
use dao::term::query::find_term;
use dao::term::term::Term;

async fn validate_term(
    State(language): State<LanguageDao>,
    Path(term): Path<Term>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let conn = get_pool().await;

    let found = find_term(conn, language, term.clone()).await;

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

pub fn term_route(language: LanguageDao, router: Router) -> Router {
    Router::new()
        .nest("/{term}", router)
        .layer(middleware::from_fn_with_state(language, validate_term))
}
