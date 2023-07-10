use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
    sync::Arc,
};

use crate::config::AppConfig;
use clap::Parser;
use dotenvy::dotenv;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
mod app;
mod authentication;
mod config;
mod database;
mod server;
mod telemetry;
mod tools;
mod types;

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let config = Arc::new(AppConfig::parse());

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(
            PgConnectOptions::new()
                .host(&config.postgres_host)
                .username(&config.postgres_user)
                .password(&config.postgres_password)
                .port(config.postgres_port)
                .database(&config.postgres_db)
                .ssl_mode(match config.postgres_require_ssl {
                    true => PgSslMode::Require,
                    _ => PgSslMode::Prefer,
                }),
        );

    let sock_addr = SocketAddr::new(
        IpAddr::from_str(&config.application_host).expect("Failed to get host"),
        config.application_port,
    );
    // run_db_migrations(&connection_pool).await;

    let app = app::app(connection_pool, config).await;

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("unable to start server");

    Ok(())
}

// async fn run_db_migrations(pool: &PgPool) {
//     sqlx::migrate!()
//         .run(pool)
//         .await
//         .expect("Unable to run migration!");
// }
