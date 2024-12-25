use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Users, users},
        DbTxn, Storage,
    },
};
use sea_orm::*;
use uuid::Uuid;

impl Storage {
    pub async fn create_user(&self, user: users::Model) -> AppResult<users::Model> {
        tracing::info!("user model: {:?}", user);

        match self.is_user_exists_by_email(&user.email).await? {
            true => {
                return Err(AppError::UserExisted(format!(
                    "User: {} already exists",
                    user.email
                )))
            }
            false => (),
        }

        //let hashed_password: String = hash_password(&input.password)?;

        let mut active_user = user.into_active_model();

        active_user.id = NotSet;
        active_user.uid = Set(Uuid::new_v4().to_string());
        active_user.created_at = Set(Some(chrono::Utc::now().into()));
        active_user.updated_at = Set(Some(chrono::Utc::now().into()));

        let created_user = active_user.insert(self.conn.as_ref()).await?;

        Ok(created_user)
    }

    pub async fn is_user_exists_by_email(&self, user_email: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::Email.eq(user_email))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }

    pub async fn get_user_by_email(&self, user_email: &str) -> AppResult<users::Model> {
        match Users::find()
            .filter(users::Column::Email.eq(user_email))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                user_email
            ))),
        }
    }

    pub async fn is_user_exists_by_address(&self, user_address: &str) -> AppResult<bool> {
        let user = Users::find()
            .filter(users::Column::Address.eq(user_address))
            .one(self.conn.as_ref())
            .await?;

        Ok(user.is_some())
    }

    pub async fn get_user_by_address(&self, user_address: &str) -> AppResult<users::Model> {
        //TODO translate address to lowercase
        match Users::find()
            .filter(users::Column::Email.eq(user_address))
            .one(self.conn.as_ref())
            .await?
        {
            Some(user) => Ok(user),
            None => Err(AppError::UserUnExisted(format!(
                "User {} has not existed",
                user_address
            ))),
        }
    }

    pub async fn update_user_address_by_email(
        &self,
        user_email: &str,
        user_addr: &str,
    ) -> AppResult<()> {
        //TODO translate address to lowercase
        match self.is_user_exists_by_address(user_addr).await? {
            true => {
                return Err(AppError::UserExisted(format!(
                    "address: {} already exists",
                    user_addr
                )))
            }
            false => (),
        }

        if let Some(user) = Users::find()
            .filter(users::Column::Email.eq(user_email))
            .one(self.conn.as_ref())
            .await?
        //.map(|l| l.into_active_model())
        {
            if user.address.is_some() {
                return Err(AppError::UserExisted(format!(
                    "User address: {} already set",
                    user_addr
                )));
            }
            let mut active_user = user.into_active_model();

            active_user.address = Set(Some(user_addr.to_string()));
            active_user.updated_at = Set(Some(chrono::Utc::now().into()));

            active_user.update(self.conn.as_ref()).await?;
        }

        Ok(())
    }

    async fn count_invited_users(&self, code: &str) -> AppResult<u64> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .count(self.conn.as_ref())
            .await?)
    }

    async fn get_invited_users(&self, code: &str) -> AppResult<Vec<users::Model>> {
        Ok(Users::find()
            .filter(users::Column::InvitedBy.eq(Some(code.to_string())))
            .all(self.conn.as_ref())
            .await?)
    }
}

impl DbTxn {
    pub async fn create_user(&self, user: users::Model) -> AppResult<users::Model> {
        let mut active_user = user.into_active_model();

        active_user.id = NotSet;
        active_user.uid = Set(Uuid::new_v4().to_string());
        active_user.created_at = Set(Some(chrono::Utc::now().into()));
        active_user.updated_at = Set(Some(chrono::Utc::now().into()));

        let created_user = active_user.insert(&self.0).await?;

        Ok(created_user)
    }
}
