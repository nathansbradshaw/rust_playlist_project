use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use secrecy::Secret;
use sqlx::types::JsonValue;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub date_created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
    pub email: String,
    pub password_hash: Option<String>,
    pub access_token: Option<String>,
    pub spotify_id: Option<String>,
    pub spotify_username: Option<String>,
    pub spotify_access_token: Option<String>,
    pub spotify_refresh_token: Option<String>,
    pub spotify_exp: Option<String>,
    pub meta: Option<JsonValue>,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            date_created: Utc::now().naive_utc(),
            last_updated: Utc::now().naive_utc(),
            email: String::from("stub email"),
            password_hash: None,
            access_token: None,
            spotify_id: None,
            spotify_username: None,
            spotify_access_token: None,
            spotify_refresh_token: None,
            spotify_exp: None,
            meta: None,
        }
    }
}

pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersRepository {
    async fn create_user(&self, email: &str, hash_password: Secret<String>)
        -> anyhow::Result<User>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
}
