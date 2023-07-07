use std::sync::Arc;
pub mod user_services;

use crate::{database::Database, server::services::user_services::UsersService};
#[derive(Clone)]
pub struct Services {
    pub users: DynUsersService,
}

use self::user_services::DynUsersService;

impl Services {
    pub fn new(db: Database) -> Self {
        // TODO change this to use tracing
        println!("Setting up services");

        let repository = Arc::new(db);

        // TODO initialize services

        let users = Arc::new(UsersService::new(repository.clone())) as DynUsersService;

        Self { users }
    }
}
