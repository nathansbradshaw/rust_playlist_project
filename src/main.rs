use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4},
    str::FromStr,
};

use configuration::get_configuration;
use sqlx::{postgres::PgPoolOptions, PgPool};
mod app;
mod configuration;
mod routes;

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sock_addr = SocketAddr::new(
        IpAddr::from_str(configuration.application.host.as_str()).expect("Failed to get host"),
        configuration.application.port,
    );
    run_db_migrations(&connection_pool).await;

    let app = app::app(connection_pool).await;

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("unable to start server");

    Ok(())
}

async fn run_db_migrations(pool: &PgPool) {
    sqlx::migrate!()
        .run(pool)
        .await
        .expect("Unable to run migration!");
}
