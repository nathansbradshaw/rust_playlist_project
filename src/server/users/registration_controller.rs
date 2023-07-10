use crate::{
    authentication::compute_password_hash,
    telemetry::spawn_blocking_with_tracing,
    tools::error_chain_fmt,
    types::{NewRegistration, UserEmail, UserPassword},
};
use anyhow::{Context, Result};
use axum::{response::IntoResponse, routing::post, Extension, Form, Router};
use hyper::StatusCode;
use secrecy::{ExposeSecret, Secret};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

pub fn routes() -> Router {
    Router::new().route("/register", post(register_post))
}

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    email: String,
    password: Secret<String>,
}

impl TryFrom<RegisterForm> for NewRegistration {
    type Error = String;

    fn try_from(value: RegisterForm) -> Result<Self, Self::Error> {
        let email = UserEmail::parse(value.email)?;
        let password = UserPassword::parse(value.password.expose_secret().to_string())?;
        Ok(Self { email, password })
    }
}

pub async fn register_post(
    Extension(pool): Extension<PgPool>,
    Form(register_form): Form<RegisterForm>,
) -> Result<impl IntoResponse, RegisterError> {
    let new_registration: NewRegistration = register_form
        .try_into()
        .map_err(RegisterError::ValidationError)?;

    register_user(new_registration, pool).await?;

    Ok(StatusCode::OK)
}

async fn register_user(new_registration: NewRegistration, pool: PgPool) -> Result<PgQueryResult> {
    let password_hash = spawn_blocking_with_tracing(move || {
        compute_password_hash(new_registration.password.into())
    })
    .await
    .context("Error hashing password")
    .map_err(RegisterError::from)?
    .context("Error hashing password")
    .map_err(RegisterError::from)?;

    let user_id = Uuid::new_v4();

    let result = sqlx::query!(
        r#"
    INSERT INTO users (id, email, password_hash)
    VALUES ($1, $2, $3)
    "#,
        user_id,
        new_registration.email.as_ref(),
        password_hash.expose_secret()
    )
    .execute(&pool)
    .await
    .unwrap();
    // .context("Failed to register user")
    // .map_err(RegisterError::from)?;

    Ok(result)
}

#[derive(thiserror::Error)]
pub enum RegisterError {
    #[error("{0}")]
    ValidationError(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for RegisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for RegisterError {
    fn into_response(self) -> axum::response::Response {
        match self {
            RegisterError::ValidationError(_) => StatusCode::BAD_REQUEST.into_response(),
            RegisterError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
