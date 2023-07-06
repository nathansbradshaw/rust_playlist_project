use playlist_project::app::app;
use reqwest::Client;

pub mod test_util {
    use axum::Router;
    use playlist_project::configuration::{get_configuration, DatabaseSettings};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Connection, Executor, PgConnection, PgPool, Pool};
    use std::net::TcpListener;
    use std::time::Duration;
    use uuid::Uuid;

    use super::*;

    pub async fn setup() -> (Client, String, Pool<sqlx::Postgres>) {
        let (app, address, listener, pool) = app_setup().await;
        start_test_server(listener, app).await;
        (reqwest::Client::new(), address, pool)
    }

    pub async fn app_setup() -> (Router, String, TcpListener, Pool<sqlx::Postgres>) {
        let configuration = {
            let mut configuration = get_configuration().expect("Failed to read configuration");
            configuration.application.port = 0;
            configuration.database.database_name = Uuid::new_v4().to_string();
            configuration
        };

        let db_pool = configure_database(&configuration.database).await;

        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
        let port = listener.local_addr().unwrap().port();
        let address = format!("http://127.0.0.1:{}", port);

        sqlx::migrate!()
            .run(&db_pool)
            .await
            .expect("Unable to run migration!");

        (app(db_pool.clone()).await, address, listener, db_pool)
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

    async fn configure_database(settings: &DatabaseSettings) -> PgPool {
        let mut connection = PgConnection::connect_with(&settings.without_db())
            .await
            .expect("Failed to connect to Postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, settings.database_name).as_str())
            .await
            .expect("Failed to create database");

        let db_pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_lazy_with(settings.with_db());

        sqlx::migrate!("./migrations")
            .run(&db_pool)
            .await
            .expect("Failed to migrate the database");

        db_pool
    }
}
