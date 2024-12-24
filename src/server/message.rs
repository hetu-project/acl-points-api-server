use crate::database::entities::users;
use serde::{Deserialize, Serialize};
use std::convert::Into;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    //pub req_id: String,
    pub code: u16,
    pub result: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    pub sub: String,
    pub name: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email_verified: bool,
}

impl Into<users::Model> for UserInfo {
    fn into(self) -> users::Model {
        users::Model {
            id: Default::default(),
            uid: "".to_string(),
            name: self.name,
            email: self.email,
            address: None,
            password: None,
            role: "user".to_string(),
            photo: self.picture,
            verified: self.email_verified,
            provider: "google".to_string(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub address: Option<String>,
    pub invite_code: String,
    pub points: u64,
    pub invite_total: u32,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            uid: user.uid,
            name: user.name,
            email: user.email,
            photo: user.photo,
            address: user.address,
            invite_code: "123456".to_string(),
            points: 123,
            invite_total: 45,
        }
    }
}