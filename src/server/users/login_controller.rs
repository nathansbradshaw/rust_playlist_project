use axum::{
    response::{AppendHeaders, IntoResponse},
    routing::post,
    Extension, Form, Router,
};
use hyper::{header::LOCATION, StatusCode};
use secrecy::Secret;
use sqlx::PgPool;

use crate::{
    authentication::{validate_credentials, AuthError, Credentials},
    tools::error_chain_fmt,
};

#[derive(serde::Deserialize)]
pub struct LoginForm {
    email: String,
    password: Secret<String>,
}

pub fn routes() -> Router {
    Router::new().route("/login", post(login_post))
}

//TODO this needs reworked
pub async fn login_post(
    Extension(pool): Extension<PgPool>,
    Form(login_form): Form<LoginForm>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let credentials = Credentials {
        email: login_form.email,
        password: login_form.password,
    };

    println!("LOGIN");

    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));
    match validate_credentials(credentials, &pool).await {
        Ok(id) => {
            tracing::Span::current().record("user_id", &tracing::field::display(&id));
            let headers = AppendHeaders([(LOCATION, "/login")]);
            Ok((headers, StatusCode::SEE_OTHER).into_response())
        }
        Err(error) => {
            let error = match error {
                AuthError::InvalidCredentials(_) => LoginError::AuthError(error.into()),
                AuthError::UnexpectedError(_) => LoginError::UnexpectedError(error.into()),
            };
            Err(login_redirect(error))
        }
    }
}

fn login_redirect(error: LoginError) -> impl IntoResponse {
    // TODO rework this function
    tracing::Span::current().record("login error", &tracing::field::display(&error));
    let headers = AppendHeaders([(LOCATION, "/login")]);
    (headers, StatusCode::SEE_OTHER).into_response()
}

#[derive(thiserror::Error)]
pub enum LoginError {
    #[error("Authentication failed")]
    AuthError(#[source] anyhow::Error),
    #[error("Something went wrong")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
