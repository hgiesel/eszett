mod extractor;

use axum::{Json, Router};
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use base::language::Language;
use dao::connection::get_pool;
use dao::language::dao::LanguageDao;
use dao::term::query::{filter_terms, find_term};
use dao::term::term::Term;
use crate::term::extractor::ExtractTerm;
use super::language::extension::language_route;
use super::language::extractor::ExtractLanguage;

async fn list_terms(
    ExtractLanguage(language): ExtractLanguage,
) -> impl IntoResponse {
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

async fn get_term(
    ExtractLanguage(language): ExtractLanguage,
    ExtractTerm(term): ExtractTerm,
) -> impl IntoResponse {
    let pool = get_pool().await;

    let dao = match language {
        Language::English => LanguageDao::English,
        Language::Spanish => LanguageDao::Spanish,
        Language::French => LanguageDao::French,
        Language::Italian => LanguageDao::Italian,
        _ => LanguageDao::English,
    };
    match find_term(&pool, dao, term).await.unwrap() {
        None => StatusCode::NOT_FOUND.into_response(),
        Some(_) => (StatusCode::OK, Json(())).into_response(),
    }
}

pub fn term_routes() -> Router {
    language_route(Router::new()
        .route("/", get(list_terms))
        .route("/{term}", get(get_term))
    )
}
