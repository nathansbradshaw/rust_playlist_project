use axum::{http::Request, Extension, Router};
use axum_session::{Key, SecurityMode, SessionConfig, SessionPgPool, SessionStore};

use crate::server::{
    health_check,
    users::{login_controller, registration_controller},
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

pub async fn app(pool: PgPool) -> Router {
    // enable console logging
    // use std::sync::Once;

    // static START: Once = Once::new();

    // START.call_once(|| {
    //     // run initialization here
    //     tracing_subscriber::fmt::init();
    // });

    // TODO find a better place for this
    let session_config = SessionConfig::default()
        .with_table_name("sessions_table")
        // 'Key::generate()' will generate a new key each restart of the server.
        // If you want it to be more permanent then generate and set it to a config file.
        // If with_key() is used it will set all cookies as private, which guarantees integrity, and authenticity.
        .with_key(Key::generate())
        // This is how we would Set a Database Key to encrypt as store our per session keys.
        // This MUST be set in order to use SecurityMode::PerSession.
        .with_database_key(Key::generate())
        // This is How you will enable PerSession SessionID Private Cookie Encryption. When enabled it will
        // Encrypt the SessionID and Storage with an Encryption key generated and stored per session.
        // This allows for Key renewing without needing to force the entire Session from being destroyed.
        // This Also helps prevent impersonation attempts.
        .with_security_mode(SecurityMode::PerSession);

    let session_store = SessionStore::<SessionPgPool>::new(None, session_config);
    session_store.initiate().await.unwrap();

    // end TODO find a better place for this

    Router::new()
        .nest("/api", login_controller::routes())
        .nest("/api", registration_controller::routes())
        .nest("/api", health_check::routes())
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
}
