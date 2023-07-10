#[derive(clap::ValueEnum, Clone, Debug, Copy)]
pub enum CargoEnv {
    Development,
    Production,
    Test,
}

#[derive(clap::Parser)]
pub struct AppConfig {
    #[clap(long, env, value_enum)]
    pub cargo_env: CargoEnv,

    #[clap(long, env, default_value = "3000")]
    pub application_port: u16,

    #[clap(long, env, default_value = "127.0.0.1")]
    pub application_host: String,

    #[clap(long, env)]
    pub run_migrations: bool,

    #[clap(long, env)]
    pub argon_salt: String,

    #[clap(long, env)]
    pub access_token_secret: String,

    #[clap(long, env)]
    pub refresh_token_secret: String,

    #[clap(long, env)]
    pub cors_origin: String,

    #[clap(long, env)]
    pub seed: bool,

    #[clap(long, env, default_value = "postgres")]
    pub postgres_user: String,

    #[clap(long, env, default_value = "password")]
    pub postgres_password: String,

    #[clap(long, env, default_value = "5432")]
    pub postgres_port: u16,

    #[clap(long, env, default_value = "127.0.0.1")]
    pub postgres_host: String,

    #[clap(long, env, default_value = "playlist_project")]
    pub postgres_db: String,

    #[clap(long, env, default_value = "false")]
    pub postgres_require_ssl: bool,
}
