use async_trait::async_trait;
use sqlx::{
    types::chrono::{NaiveDateTime, Utc},
    FromRow,
};
use std::sync::Arc;
use uuid::{uuid, Uuid};

use crate::database::user::User;

#[derive(FromRow, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub exp: NaiveDateTime,
    pub user_agent: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            id: uuid!("8147a9f8-2845-4f92-9e1d-0c0c6c8db79b"),
            user_id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            exp: Utc::now().naive_utc(),
            user_agent: String::from("stub user agent"),
        }
    }
}

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynSessionsRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[async_trait]
pub trait SessionsRepository {
    async fn new_session(
        &self,
        user_id: Uuid,
        user_agent: &str,
        exp: &NaiveDateTime,
    ) -> anyhow::Result<Session>;

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}
