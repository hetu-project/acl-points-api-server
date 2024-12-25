use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Rewards, prelude::RewardsPool, rewards, rewards_pool},
        DbTxn, Storage,
    },
};
use sea_orm::*;

impl Storage {
    pub async fn claim_reward(
        &self,
        user_uid: i32,
        reward_type: &str,
        claim_amount: i32,
    ) -> AppResult<()> {
        if let Some(reward) = RewardsPool::find()
            .filter(rewards_pool::Column::RewardType.eq(reward_type))
            .one(self.conn.as_ref())
            .await?
        {
            let available_amount = reward.available_amount.clone();
            if available_amount < claim_amount {
                return Err(AppError::CustomError("Not enough rewards available".into()));
            }

            let mut active_reward = reward.into_active_model();
            active_reward.available_amount = Set(available_amount - claim_amount);
            active_reward.updated_at = Set(chrono::Utc::now().into());

            active_reward.update(self.conn.as_ref()).await?;
        }

        let user_reward = rewards::ActiveModel {
            id: NotSet,
            user_uid: Set(user_uid),
            reward_type: Set(reward_type.to_owned()),
            amount: Set(claim_amount),
            created_at: Set(chrono::Utc::now().into()),
        };

        user_reward.insert(self.conn.as_ref()).await?;

        Ok(())
    }

    pub async fn update_rewards_pool(&self, reward_type: &str, total_amount: i32) -> AppResult<()> {
        let reward_pool = RewardsPool::find()
            .filter(rewards_pool::Column::RewardType.eq(reward_type))
            .one(self.conn.as_ref())
            .await?;

        match reward_pool {
            Some(pool) => {
                if pool.available_amount > total_amount {
                    return Err(AppError::CustomError("invalid total amount".into()));
                }

                let mut active_pool = pool.into_active_model();
                active_pool.total_amount = Set(total_amount);
                active_pool.updated_at = Set(chrono::Utc::now().into());
                active_pool.update(self.conn.as_ref()).await?;
            }
            None => {
                let new_pool = rewards_pool::ActiveModel {
                    reward_type: Set(reward_type.to_owned()),
                    total_amount: Set(total_amount),
                    available_amount: Set(total_amount),
                    updated_at: Set(chrono::Utc::now().into()),
                    ..Default::default()
                };

                new_pool.insert(self.conn.as_ref()).await?;
            }
        }

        Ok(())
    }
}
