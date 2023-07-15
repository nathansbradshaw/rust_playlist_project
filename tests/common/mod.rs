pub mod types;
use playlist_project::app::app;
use reqwest::Client;

pub mod test_util {
    use super::*;
    use axum::Router;
    use playlist_project::config::{AppConfig, CargoEnv};

    use sqlx::Pool;
    use std::{net::TcpListener, sync::Arc};

    pub async fn setup(pool: sqlx::Pool<sqlx::Postgres>) -> (Client, String, Pool<sqlx::Postgres>) {
        let (app, address, listener, pool) = app_setup(pool).await;

        start_test_server(listener, app).await;
        (
            reqwest::Client::builder()
                .cookie_store(true)
                .user_agent("Value")
                .build()
                .expect("Failed building client"),
            address,
            pool,
        )
    }

    pub async fn app_setup(
        db_pool: sqlx::Pool<sqlx::Postgres>,
    ) -> (Router, String, TcpListener, Pool<sqlx::Postgres>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        sqlx::migrate!()
            .run(&db_pool)
            .await
            .expect("Unable to run migration!");

        let app_config = AppConfig {
            cargo_env: CargoEnv::Test,
            application_port: port,
            application_host: "127.0.0.1".to_string(),
            run_migrations: true,
            argon_salt: "Saltyboi".to_string(),
            access_token_secret: "secret_access".to_string(),
            refresh_token_secret: "secret_refresh".to_string(),
            cors_origin: "*".to_string(),
            seed: true,
            // this data isn't used in tests
            postgres_user: "".to_string(),
            postgres_password: "".to_string(),
            postgres_port: 0000,
            postgres_host: "".to_string(),
            postgres_db: "".to_string(),
            postgres_require_ssl: true,
        };
        let config = Arc::new(app_config);

        (
            app(db_pool.clone(), config).await,
            address,
            listener,
            db_pool,
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
}
