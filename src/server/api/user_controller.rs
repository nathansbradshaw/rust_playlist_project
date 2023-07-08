use axum::{routing::post, Extension, Json, Router};

use crate::server::{
    dtos::user_dto::SignUpUserDto, error::AppResult,
    extractors::validation_extractor::ValidationExtractor, services::Services,
};

pub struct UserController;

impl UserController {
    pub fn app() -> Router {
        Router::new().route("/signup", post(Self::signup_user_endpoint))
    }

    pub async fn signup_user_endpoint(
        Extension(services): Extension<Services>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<()> {
        let _created_user = services.users.signup_user(request).await?;

        Ok(())
    }
}
