use anyhow::Context;
use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

use super::{User, UsersRepository};

#[async_trait]
impl UsersRepository for PgPool {
    async fn create_user(
        &self,
        email: &str,
        hash_password: Secret<String>,
    ) -> anyhow::Result<User> {
        println!("Creating user");
        let user_id = Uuid::new_v4();
        query_as!(
            User,
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
        returning *
            "#,
            user_id,
            email,
            hash_password.expose_secret()
        )
        .fetch_one(self)
        .await
        .context("an unexpected error occured while creating the user")
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
            r#"
        select *
        from users
        where email = $1::varchar
            "#,
            email,
        )
        .fetch_optional(self)
        .await
        .context("unexpected error while querying for user by email")
    }

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        select *
        from users
        where id = $1
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
        email: String,
        password: Option<String>,
    ) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        update users
        set
            email = $1::varchar,
            password_hash = $2::varchar,
            last_updated = current_timestamp
        where id = $3
        returning *
            "#,
            email,
            password,
            id
        )
        .fetch_one(self)
        .await
        .context("could not update the user")
    }
}
