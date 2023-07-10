use super::{user_email::UserEmail, user_password::UserPassword};

pub struct NewRegistration {
    pub email: UserEmail,
    pub password: UserPassword,
}
