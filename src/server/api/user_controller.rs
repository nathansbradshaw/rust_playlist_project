use crate::server::{
    dtos::user_dto::{SignInUserDto, SignUpUserDto, UpdateUserDto, UserAuthenicationResponse},
    error::AppResult,
    extractors::{
        required_authentication_extractor::RequiredAuthentication,
        session_extractor::SessionExtractor, user_agent_extractor::UserAgentExtractor,
        validation_extractor::ValidationExtractor,
    },
    services::Services,
};
use axum::{routing::*, Extension, Json, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::info;

pub struct UserController;

impl UserController {
    pub fn app() -> Router {
        Router::new()
            .route("/signup", post(Self::signup_user_endpoint))
            .route("/signin", post(Self::signin_user_endpoint))
            .route("/signout", post(Self::signout_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/refresh", get(Self::refresh_user_endpoint))
            .route("/", put(Self::update_user_endpoint))
    }

    pub async fn signup_user_endpoint(
        Extension(services): Extension<Services>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}",
            request.email.as_ref().unwrap(),
        );

        let req = services.users.signup_user(request).await;

        println!("Request: {:?}", req);

        let created_user = req?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        println!("Signin user");

        let (user, refresh_token) = services.users.signin_user(request, user_agent).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = services.users.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
        Json(request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = services.users.updated_user(user_id, request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }

    pub async fn refresh_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, refresh_token): SessionExtractor,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!("recieved request to refresh access token {:?}", session_id);

        let user = services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn signout_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session {:?}", session_id);

        services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.remove(Cookie::named("refresh_token"));

        Ok(cookie)
    }
}
