use playlist_project::app::app;
use reqwest::Client;

pub mod test_util {
    use super::*;
    use axum::Router;
    use clap::Parser;
    use dotenvy::dotenv;
    use playlist_project::config::AppConfig;
    use sqlx::Pool;
    use std::{net::TcpListener, sync::Arc};

    pub async fn setup(pool: sqlx::Pool<sqlx::Postgres>) -> (Client, String, Pool<sqlx::Postgres>) {
        let (app, address, listener, pool) = app_setup(pool).await;
        start_test_server(listener, app).await;
        (reqwest::Client::new(), address, pool)
    }

    pub async fn app_setup(
        db_pool: sqlx::Pool<sqlx::Postgres>,
    ) -> (Router, String, TcpListener, Pool<sqlx::Postgres>) {
        //TODO change this as it will fail in the ci/cd once .env is removed
        dotenv().ok();
        let config = Arc::new(AppConfig::parse());

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        sqlx::migrate!()
            .run(&db_pool)
            .await
            .expect("Unable to run migration!");

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
