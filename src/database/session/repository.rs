use anyhow::Context;
use async_trait::async_trait;
use sqlx::{
    query_as,
    types::chrono::{DateTime, Utc},
    PgPool,
};
use uuid::Uuid;

use super::{Session, SessionsRepository};
use crate::database::user::User;
use crate::types::{HashedPassword, UserEmail};

#[async_trait]
impl SessionsRepository for PgPool {
    async fn new_session(
        &self,
        user_id: Uuid,
        user_agent: &str,
        exp: &DateTime<Utc>,
    ) -> anyhow::Result<Session> {
        query_as!(
            Session,
            r#"
                INSERT INTO sessions (user_id,user_agent,exp)
                VALUES ($1,$2,$3)
                RETURNING *
            "#,
            user_id,
            user_agent,
            exp
        )
        .fetch_one(self)
        .await
        .context("an unexpected error occured while creating a session")
        // todo!()
    }

    async fn get_user_by_session_id(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
                SELECT 
                    users.id,
                    users.date_created,
                    users.last_updated,
                    users.password_hash as "password_hash: HashedPassword",
                    users.access_token,
                    users.spotify_id,
                    users.spotify_username,
                    users.spotify_access_token,
                    users.spotify_refresh_token,
                    users.spotify_exp,
                    users.meta,
                    users.email AS "email: UserEmail"
                FROM users
                INNER JOIN sessions
                ON users.id = sessions.user_id
                WHERE sessions.exp >= now() AND sessions.id = $1
            "#,
            id,
        )
        .fetch_optional(self)
        .await
        .context("user was not found")
    }
}
