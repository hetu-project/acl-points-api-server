use crate::common::error::{AppError, AppResult};
use serde::Deserialize;
use validator::Validate;

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
