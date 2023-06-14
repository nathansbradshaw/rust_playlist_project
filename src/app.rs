use axum::{Extension, Router};

use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::routes::health_check;

pub async fn app(pool: PgPool) -> Router {
    // enable console logging
    use std::sync::Once;

    static START: Once = Once::new();

    START.call_once(|| {
        // run initialization here
        tracing_subscriber::fmt::init();
    });

    Router::new()
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        .layer(Extension(pool))
        .nest("/api", health_check::routes())
}

#[cfg(test)]
pub mod test_util_app {
    use std::net::TcpListener;

    use axum::Router;
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use sqlx::Pool;
    use testcontainers::clients;
    use testcontainers::images::postgres::Postgres;

    use super::*;

    pub async fn setup() -> (Router, String, TcpListener, Pool<sqlx::Postgres>) {
        let docker = clients::Cli::default();
        let postgres = docker.run(Postgres::default());

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        let connection_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(
                PgConnectOptions::new()
                    .host("127.0.0.1")
                    .port(postgres.get_host_port_ipv4(5432))
                    .username("postgres")
                    .password("password"),
            );

        (
            app(connection_pool.clone()).await,
            address,
            listener,
            connection_pool,
        )
    }
}
