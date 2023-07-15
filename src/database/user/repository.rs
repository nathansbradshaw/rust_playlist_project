use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::types::{HashedPassword, UserEmail};

use super::{User, UsersRepository};

#[async_trait]
impl UsersRepository for PgPool {
    async fn create_user(
        &self,
        email: &UserEmail,
        hash_password: HashedPassword,
    ) -> anyhow::Result<User> {
        println!("Creating user");
        let user_id = Uuid::new_v4();
        query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING 
                id,
                date_created,
                last_updated,
                password_hash as "password_hash: HashedPassword",
                access_token,
                spotify_id,
                spotify_username,
                spotify_access_token,
                spotify_refresh_token,
                spotify_exp,
                meta,
                email AS "email: UserEmail"
            "#,
            user_id,
            email.as_ref(),
            String::from(hash_password),
        )
        .fetch_one(self)
        .await
        .context("an unexpected error occured while creating the user")
    }

    async fn get_user_by_email(&self, email: &UserEmail) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
                SELECT 
                    id,
                    date_created,
                    last_updated,
                    password_hash as "password_hash: HashedPassword",
                    access_token,
                    spotify_id,
                    spotify_username,
                    spotify_access_token,
                    spotify_refresh_token,
                    spotify_exp,
                    meta,
                    email AS "email: UserEmail"
                FROM users
                WHERE email = $1::varchar
            "#,
            email.as_ref()
        )
        .fetch_optional(self)
        .await
        .context("unexpected error while querying for user by email")
    }

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
                SELECT
                    id,
                    date_created,
                    last_updated,
                    password_hash as "password_hash: HashedPassword",
                    access_token,
                    spotify_id,
                    spotify_username,
                    spotify_access_token,
                    spotify_refresh_token,
                    spotify_exp,
                    meta,
                    email AS "email: UserEmail"
                FROM users
                WHERE id = $1
            "#,
            id,
        )
        .fetch_one(self)
        .await
        .context("user was not found")
    }

    async fn update_user(
        &self,
        id: Uuid,
        email: &UserEmail,
        password: Option<HashedPassword>,
    ) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
                UPDATE users
                SET
                    email = $1::varchar,
                    password_hash = $2::varchar,
                    last_updated = current_timestamp
                WHERE id = $3
                RETURNING 
                    id,
                    date_created,
                    last_updated,
                    password_hash as "password_hash: HashedPassword",
                    access_token,
                    spotify_id,
                    spotify_username,
                    spotify_access_token,
                    spotify_refresh_token,
                    spotify_exp,
                    meta,
                    email AS "email: UserEmail"
            "#,
            email.as_ref(),
            match password {
                Some(p) => Some(String::from(p)),
                None => None,
            },
            id
        )
        .fetch_one(self)
        .await
        .context("could not update the user")
    }
}
