use axum::Router;
mod user_controller;

use self::user_controller::UserController;

pub fn app() -> Router {
    Router::new().nest("/users", UserController::app())
}
