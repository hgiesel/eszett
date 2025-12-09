use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, MethodRouter};
use dao::connection::init_pool;
use dao::language::dao::LanguageDao;
use dao::term::query::filter_terms;

async fn list_terms() -> (StatusCode, Json<Vec<String>>) {
    let pool = init_pool().await;
    let result = filter_terms(&pool, LanguageDao::English).await.unwrap();
    println!("Hello, world! trying to list terms");
    (StatusCode::OK, Json(result.into_iter().map(|term| term.term).collect()))
}

pub fn term_routes() -> Router {
    Router::new()
}
