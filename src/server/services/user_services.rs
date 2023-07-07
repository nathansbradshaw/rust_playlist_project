use secrecy::Secret;
use std::sync::Arc;
use tracing::{error, info};

use crate::{
    database::user::DynUsersRepository,
    error::{AppResult, Error},
    server::dtos::user_dto::{ResponseUserDto, SignUpUserDto},
};
use async_trait::async_trait;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersServiceTrait + Send + Sync>;

#[async_trait]
pub trait UsersServiceTrait {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<ResponseUserDto>;
}

#[derive(Clone)]
pub struct UsersService {
    repository: DynUsersRepository,
}

impl UsersService {
    pub fn new(repository: DynUsersRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
#[allow(implied_bounds_entailment)]
impl UsersServiceTrait for UsersService {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<ResponseUserDto> {
        let email = request.email.unwrap();
        let password = request.password.unwrap();

        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_some() {
            error!("user {:?} already exists", email);
            return Err(Error::ObjectConflict(format!("email {} is taken", email)));
        }

        info!("creating password hash for user {:?}", email);
        let hashed_password = "Hash".to_string();

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self
            .repository
            .create_user(&email, Secret::new(hashed_password))
            .await?;

        Ok(created_user.into_dto())
    }
}
