use playlist_project::app::app;
use reqwest::Client;

pub mod test_util {
    use axum::Router;
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
    use sqlx::Pool;
    use std::net::TcpListener;
    use testcontainers::clients;
    use testcontainers::images::postgres::Postgres;

    use super::*;

    pub async fn app_setup() -> (Router, String, TcpListener, Pool<sqlx::Postgres>) {
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
    pub async fn start_test_server(listener: TcpListener, app: Router) -> () {
        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(app.into_make_service())
                .await
                .unwrap();
        });
    }

    pub async fn setup() -> (Client, String, Pool<sqlx::Postgres>) {
        let (app, address, listener, pool) = app_setup().await;
        start_test_server(listener, app).await;
        (reqwest::Client::new(), address, pool)
    }
}
