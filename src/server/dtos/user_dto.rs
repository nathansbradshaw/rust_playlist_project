use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::user::User;

impl User {
    pub fn into_dto(self, token: String) -> ResponseUserDto {
        ResponseUserDto {
            id: self.id,
            email: self.email,
            access_token: Some(token),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseUserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: ResponseUserDto,
}

// impl UserAuthenicationResponse {
//     pub fn new(
//         id: Uuid,
//         name: String,
//         email: String,
//         // unfortunately, while our implementation returns thes optional fields as empty strings,
//         // the realworld demo API enables nullable serializing by default, so we have to wrap these
//         // strings as `Option` option values for now
//         access_token: Option<String>,
//     ) -> Self {
//         UserAuthenicationResponse {
//             user: ResponseUserDto {
//                 id,
//                 email,
//                 access_token,
//             },
//         }
//     }
// }

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignUpUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignInUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

// impl SignUpUserDto {
//     pub fn new_stub() -> Self {
//         Self {
//             email: Some(String::from("stub email")),
//             password: Some(String::from("stub password")),
//         }
//     }
// }

// impl SignInUserDto {
//     pub fn new_stub() -> Self {
//         Self {
//             email: Some(String::from("stub email")),
//             password: Some(String::from("stub password")),
//         }
//     }
// }
