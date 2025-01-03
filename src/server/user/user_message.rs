use super::user_service;
use crate::{
    common::error::{AppError, AppResult},
    database::entities::users,
    server::auth::OauthUserInfo,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use std::convert::Into;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct PointsResponse {
    pub point: u64,
    pub invite_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountResponse {
    pub count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub password: Option<String>,
    pub invited_by: Option<String>,
    pub invite_code: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub provider: String,
}

impl From<OauthUserInfo> for User {
    fn from(item: OauthUserInfo) -> Self {
        Self {
            uid: user_service::gen_uid(),
            name: item.name,
            email: item.email,
            address: None,
            password: None,
            invited_by: None,
            invite_code: user_service::gen_invite_code(8),
            role: "user".to_string(),
            photo: item.picture,
            verified: item.email_verified,
            provider: "google".to_string(),
        }
    }
}

impl User {
    pub fn add_invited_by(mut self, invited: &str) -> Self {
        self.invited_by = Some(invited.to_string());
        self
    }
}

impl Into<users::ActiveModel> for User {
    fn into(self) -> users::ActiveModel {
        users::ActiveModel {
            id: NotSet,
            uid: Set(self.uid),
            name: Set(self.name),
            email: Set(self.email),
            address: Set(self.address),
            password: Set(self.password),
            invited_by: Set(self.invited_by),
            invite_code: Set(self.invite_code),
            role: Set(self.role),
            photo: Set(self.photo),
            verified: Set(self.verified),
            provider: Set(self.provider),
            created_at: Set(Some(chrono::Utc::now().into())),
            updated_at: Set(Some(chrono::Utc::now().into())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserResponse {
    pub uid: String,
    pub name: String,
    pub email: String,
    pub photo: String,
    pub invite_code: String,
    pub invited_by: Option<String>,
}

impl From<users::Model> for UserResponse {
    fn from(user: users::Model) -> Self {
        Self {
            uid: user.uid,
            name: user.name,
            email: user.email,
            photo: user.photo,
            invite_code: user.invite_code,
            invited_by: user.invited_by,
        }
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct UpdateAddressReq {
    #[validate(length(min = 64, max = 66, message = "address has a wrong format"))]
    pub address: Option<String>,
}

impl UpdateAddressReq {
    pub fn validate_items(&self) -> AppResult<()> {
        if self.address.is_none() {
            return Err(AppError::CustomError("code not found".to_string()));
        }

        //if len == 66, must has prefix '0x'
        Ok(self.validate()?)
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct ComfirmReq {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(length(min = 1, max = 64, message = "uid has a wrong format"))]
    pub uid: Option<String>,
}

impl ComfirmReq {
    pub fn validate_items(&self) -> AppResult<()> {
        Ok(self.validate()?)
    }
}
