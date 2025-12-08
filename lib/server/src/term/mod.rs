use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::get;
use eszett_dao::connection::init_pool;
use eszett_dao::term::query::filter_terms;
use crate::{CreateUser, User};

async fn list_terms() -> (StatusCode, Json<Vec<String>>) {
    let pool = init_pool().await;
    let result = filter_terms(&pool, 1).await.unwrap();
    println!("Hello, world! trying to list terms");
    (StatusCode::OK, Json(result.into_iter().map(|term| term.term).collect()))
}

pub fn term_routes() -> Router {
    Router::new()
        .route("/", get(list_terms))
}
