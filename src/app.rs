use std::sync::Arc;

use axum::{http::Request, Extension, Router};

use crate::{
    config::AppConfig,
    server::{api, health_check, services::Services},
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, RequestId},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use tracing::Level;
use uuid::Uuid;

#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string();

        Some(RequestId::new(request_id.parse().unwrap()))
    }
}

pub async fn app(pool: PgPool, config: Arc<AppConfig>) -> Router {
    // enable console logging
    // use std::sync::Once;

    // static START: Once = Once::new();

    // START.call_once(|| {
    //     // run initialization here
    //     tracing_subscriber::fmt::init();
    // });

    let services = Services::new(pool.clone(), config);

    Router::new()
        .nest("/api/v1", api::app())
        .nest("/api/v1", health_check::routes())
        .layer(
            // from https://docs.rs/tower-http/0.2.5/tower_http/request_id/index.html#using-trace
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(
                            DefaultMakeSpan::new()
                                .include_headers(true)
                                .level(Level::INFO),
                        )
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
        .layer(Extension(pool))
        .layer(ServiceBuilder::new().layer(Extension(services)))
}
