use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::user::User;

impl User {
    pub fn into_dto(self) -> ResponseUserDto {
        ResponseUserDto {
            id: self.id,
            email: self.email,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseUserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignUpUserDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

impl SignUpUserDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub name")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
