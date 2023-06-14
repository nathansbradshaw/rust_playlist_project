use playlist_project::app::app;
// use playlist_project::app::test_util_app;

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

#[tokio::test]
async fn health_check_works() {
    let (app, address, listener, _) = test_util_app::setup().await;
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/api/health", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
