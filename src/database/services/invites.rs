use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{invites, prelude::Invites},
        DbTxn, Storage,
    },
};
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::*;

fn generate_invite_code(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

impl Storage {
    async fn create_invite_code(&self, user_uid: &str) -> AppResult<invites::Model> {
        if let Some(_) = Invites::find_by_id(user_uid)
            .one(self.conn.as_ref())
            .await?
        {
            return Err(AppError::UserExisted(format!(
                "User's invite code: {} already exists",
                user_uid
            )));
        }

        let invited_code = loop {
            let code = generate_invite_code(8);
            let existing = Invites::find()
                .filter(invites::Column::Code.eq(code.as_str()))
                .one(self.conn.as_ref())
                .await?;

            if existing.is_none() {
                break code;
            }
        };

        let invite = invites::ActiveModel {
            user_uid: Set(user_uid.to_string()),
            code: Set(invited_code.clone()),
            created_at: Set(chrono::Utc::now().into()),
        };
        let created_code = invite.insert(self.conn.as_ref()).await?;

        Ok(created_code)
    }

    async fn get_invite_by_user(&self, user_uid: &str) -> AppResult<invites::Model> {
        match Invites::find_by_id(user_uid)
            .one(self.conn.as_ref())
            .await?
        {
            Some(invite) => Ok(invite),
            None => Err(AppError::CustomError("invite error".to_string())), //TODO
        }
    }

    async fn get_invite_by_code(&self, code: &str) -> AppResult<invites::Model> {
        match Invites::find()
            .filter(invites::Column::Code.eq(code))
            .one(self.conn.as_ref())
            .await?
        {
            Some(invite) => Ok(invite),
            None => Err(AppError::CustomError("invite error".to_string())), //TODO
        }
    }
}

impl DbTxn {
    async fn create_invite_code(
        &self,
        user_uid: &str,
        invited_code: &str,
    ) -> AppResult<invites::Model> {
        let invite = invites::ActiveModel {
            user_uid: Set(user_uid.to_string()),
            code: Set(invited_code.to_string()),
            created_at: Set(chrono::Utc::now().into()),
        };
        let created_code = invite.insert(&self.0).await?;

        Ok(created_code)
    }
}
