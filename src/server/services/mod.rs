use std::sync::Arc;
pub mod session_services;
pub mod user_services;

use sqlx::PgPool;

use crate::{
    config::AppConfig,
    server::{
        services::{
            session_services::{DynSessionsService, SessionsService},
            user_services::UsersService,
        },
        utils::{
            argon_utils::{ArgonSecurityUtil, DynArgonUtil},
            jwt_utils::JwtTokenUtil,
        },
    },
};
#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil,
    pub users: DynUsersService,
    pub sessions: DynSessionsService,
}

use self::user_services::DynUsersService;

use super::utils::jwt_utils::DynJwtUtil;

impl Services {
    pub fn new(db: PgPool, config: Arc<AppConfig>) -> Self {
        // TODO change this to use tracing
        println!("Setting up services");
        let security_service = Arc::new(ArgonSecurityUtil::new()) as DynArgonUtil;
        let jwt_util = Arc::new(JwtTokenUtil::new(config)) as DynJwtUtil;

        let repository = Arc::new(db);

        // TODO initialize services
        let sessions = Arc::new(SessionsService::new(repository.clone(), jwt_util.clone()))
            as DynSessionsService;

        let users = Arc::new(UsersService::new(
            repository,
            security_service,
            jwt_util.clone(),
            sessions.clone(),
        )) as DynUsersService;

        Self {
            jwt_util,
            users,
            sessions,
        }
    }
}
