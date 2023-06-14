use axum::{response::IntoResponse, routing::get, Router};
use reqwest::StatusCode;

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[cfg(test)]
mod test {
    use crate::app::test_util_app;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_retun_200_ok() {
        let (app, _address, _listener, _) = test_util_app::setup().await;
        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
