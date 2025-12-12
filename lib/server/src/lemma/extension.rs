use axum::extract::{Path, Request};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{Router, middleware};

async fn validate_lemma(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    todo!()
}

pub fn lemma_route(router: Router) -> Router {
    Router::new()
        .nest("/{lemma}", router)
        .layer(middleware::from_fn(validate_lemma))
}
