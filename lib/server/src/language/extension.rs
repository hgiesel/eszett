use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware};
use base::language::Language;
use serde::Deserialize;

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
        _ => None,
    };

    match result {
        Some(lang) => {
            req.extensions_mut().insert(lang);
            Ok(next.run(req).await)
        }
        None => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn language_route(router: Router) -> Router {
    Router::new()
        .nest("/{language}", router)
        .route_layer(middleware::from_fn(validate_language))
}
