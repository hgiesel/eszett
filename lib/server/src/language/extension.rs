use std::collections::HashMap;
use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{middleware, Router};
use serde::Deserialize;
use base::language::Language;

#[derive(Deserialize)]
struct LanguageName { language: String }

async fn validate_language(
    Path(LanguageName { language }): Path<LanguageName>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let result = match language.as_str() {
        "en" => Some(Language::English),
        "es" => Some(Language::Spanish),
        "fr" => Some(Language::French),
        "it" => Some(Language::Italian),
        _ => None
    };

    if result.is_some() {
        // State is set up here in the middleware
        req.extensions_mut().insert(result.unwrap());
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn language_route(router: Router) -> Router {
    Router::new()
        .nest("/{language}", router)
        .layer(middleware::from_fn(validate_language))
}
