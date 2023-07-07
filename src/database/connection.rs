use anyhow::{Context, Ok};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type ConnectionPool = Pool<Postgres>;

/// Postgres database
#[derive(Debug, Clone)]
pub struct Database {
    pub pool: ConnectionPool,
}

impl Database {
    pub async fn connect(connection_string: &str, run_migrations: bool) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("error initializing the database")?;

        if run_migrations {
            //TODO add log
            info!("migrations enabled, running...");
            sqlx::migrate!()
                .run(&pool)
                .await
                .context("error running database migration");
            info!("migrations successfully ran, initializing axum server...");
        }

        Ok(Self { pool })
    }
}
