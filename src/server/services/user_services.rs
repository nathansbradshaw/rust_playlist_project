use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    database::user::DynUsersRepository,
    server::{
        dtos::user_dto::{ResponseUserDto, SignUpUserDto, UpdateUserDto},
        utils::{argon_utils::DynArgonUtil, jwt_utils::DynJwtUtil},
    },
    server::{
        dtos::{session_dto::NewSessionDto, user_dto::SignInUserDto},
        error::{AppResult, Error},
    },
    types::HashedPassword,
};
use async_trait::async_trait;

use super::session_services::DynSessionsService;

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynUsersService = Arc<dyn UsersServiceTrait + Send + Sync>;

#[async_trait]
pub trait UsersServiceTrait {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<ResponseUserDto>;
    async fn signin_user(
        &self,
        request: SignInUserDto,
        user_agent: Option<String>,
    ) -> AppResult<(ResponseUserDto, String)>;

    async fn get_current_user(&self, user_id: Uuid) -> AppResult<ResponseUserDto>;

    async fn updated_user(
        &self,
        user_id: Uuid,
        request: UpdateUserDto,
    ) -> AppResult<ResponseUserDto>;
}

#[derive(Clone)]
pub struct UsersService {
    repository: DynUsersRepository,
    argon_util: DynArgonUtil,
    jwt_util: DynJwtUtil,
    session_service: DynSessionsService,
}

impl UsersService {
    pub fn new(
        repository: DynUsersRepository,
        argon_util: DynArgonUtil,
        jwt_util: DynJwtUtil,
        session_service: DynSessionsService,
    ) -> Self {
        Self {
            repository,
            argon_util,
            jwt_util,
            session_service,
        }
    }
}

#[async_trait]
#[allow(implied_bounds_entailment)]
impl UsersServiceTrait for UsersService {
    async fn signup_user(&self, request: SignUpUserDto) -> AppResult<ResponseUserDto> {
        let email = request.email.unwrap();
        let password = request.password.unwrap();
        println!("getting user");

        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_some() {
            error!("user {:?} already exists", email);
            return Err(Error::ObjectConflict(format!("email {} is taken", email)));
        }
        println!("got user");
        info!("creating password hash for user {:?}", email);
        let hashed_password = self.argon_util.hash_password(&password)?;
        println!("hashed password");

        info!("password hashed successfully, creating user {:?}", email);
        let created_user = self.repository.create_user(&email, hashed_password).await?;

        Ok(created_user.into_dto(String::new()))
    }

    async fn signin_user(
        &self,
        request: SignInUserDto,
        user_agent: Option<String>,
    ) -> AppResult<(ResponseUserDto, String)> {
        let email = request.email.unwrap();
        let attempted_password = request.password.unwrap();

        info!("searching for existing user {:?}", email);
        let existing_user = self.repository.get_user_by_email(&email).await?;

        if existing_user.is_none() {
            return Err(Error::NotFound(String::from("user email does not exist")));
        }

        let user = existing_user.unwrap();

        info!("user found, verifying password hash for user {:?}", email);
        let is_valid_login_attempt = self
            .argon_util
            .verify_password(user.password_hash.clone(), attempted_password)?;

        if !is_valid_login_attempt {
            error!("invalid login attempt for user {:?}", email);
            return Err(Error::InvalidLoginAttmpt);
        }

        info!("user login successful, generating tokens");

        let token = self
            .session_service
            .new_session(NewSessionDto {
                user_id: Some(user.id),
                user_agent,
            })
            .await
            .unwrap();

        Ok((user.into_dto(token.access_token), token.refresh_token))
    }

    async fn get_current_user(&self, user_id: Uuid) -> AppResult<ResponseUserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;

        info!(
            "user found with email {:?}, generating new token",
            user.email
        );
        let token = self.jwt_util.new_access_token(user.id, &user.email)?;

        Ok(user.into_dto(token))
    }

    async fn updated_user(
        &self,
        user_id: Uuid,
        request: UpdateUserDto,
    ) -> AppResult<ResponseUserDto> {
        info!("retrieving user {:?}", user_id);
        let user = self.repository.get_user_by_id(user_id).await?;
        // TODO fix these unwraps
        let updated_email = request.email.unwrap_or(user.email);
        let mut updated_hashed_password: Option<HashedPassword> = user.password_hash;

        // if the password is included on the request, hash it and update the stored password
        if request.password.is_some() && !request.password.as_ref().unwrap().as_ref().is_empty() {
            info!(
                "new password found for user {:?}, hashing password",
                user_id
            );
            updated_hashed_password = self
                .argon_util
                .hash_password(&request.password.unwrap())
                .ok()
        }

        info!("updating user {:?}", user_id);
        let updated_user = self
            .repository
            .update_user(user_id, &updated_email, updated_hashed_password)
            .await?;

        info!("user {:?} updated, generating a new token", user_id);
        let token = self.jwt_util.new_access_token(user_id, &updated_email)?;

        Ok(updated_user.into_dto(token))
    }
}
