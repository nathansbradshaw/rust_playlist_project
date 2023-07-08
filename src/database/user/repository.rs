use anyhow::Context;
use async_trait::async_trait;
use secrecy::{ExposeSecret, Secret};
use sqlx::{query_as, PgPool};

use super::{User, UsersRepository};

#[async_trait]
impl UsersRepository for PgPool {
    async fn create_user(
        &self,
        email: &str,
        hash_password: Secret<String>,
    ) -> anyhow::Result<User> {
        query_as!(
            User,
            r#"
        insert into users (email, password_hash)
        values ($1::varchar, $2::varchar)
        returning *
            "#,
            email,
            hash_password.expose_secret()
        )
        .fetch_one(&*self)
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
        .fetch_optional(&*self)
        .await
        .context("unexpected error while querying for user by email")
    }
}
