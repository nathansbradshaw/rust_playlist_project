use reqwest::Client;
use sqlx::PgPool;

pub struct SetupResponse {
    pub client: Client,
    pub address: String,
    pub pool: PgPool,
}

// This will later be split out into a shared tyes folder when this project gets converted to a mono repo
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SignUpUserDto {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResponseUserDto {
    pub email: Option<String>,
    pub access_token: Option<String>,
}
