use axum::{response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
