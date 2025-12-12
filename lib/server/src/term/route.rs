use crate::language::extension::language_route;
use crate::language::extractor::ExtractLanguage;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use base::language::Language;
use dao::connection::get_pool;
use dao::language::dao::LanguageDao;
use dao::term::query::filter_terms;
use dao::term::term::Term;
use crate::term::extension::term_route;

async fn list_terms(ExtractLanguage(language): ExtractLanguage) -> impl IntoResponse {
    let pool = get_pool().await;
    let dao = match language {
        Language::English => LanguageDao::English,
        Language::Spanish => LanguageDao::Spanish,
        Language::French => LanguageDao::French,
        Language::Italian => LanguageDao::Italian,
        _ => LanguageDao::English,
    };

    let result = filter_terms(&pool, dao).await.unwrap();
    let json = Json(result.iter().map(Term::to_string).collect::<Vec<_>>());
    (StatusCode::OK, json)
}

pub fn term_routes() -> Router {
    language_route(
        Router::new()
            .route("/", get(list_terms))
            .merge(term_route(LanguageDao::English, Router::new()
                .route("/", get(async || (StatusCode::OK, Json(()))))
            ))
    )
}
