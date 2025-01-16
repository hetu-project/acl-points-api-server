//use super::kasks::TaskRule;
use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{candy, prelude::Candy, prelude::Tasks, tasks},
        Storage,
    },
};
use once_cell::sync::OnceCell;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(FromQueryResult, Debug)]
struct AggregationResult {
    total_candies: Option<i64>, // Match the alias name
}

static CANDY_TASK: OnceCell<RwLock<CandyTask>> = OnceCell::new();

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandyTaskRule {
    pub reward_min: i32,
    pub reward_max: i32,
    pub max_attempts_per_day: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandyTask {
    pub name: String,
    pub description: String,
    pub rule: CandyTaskRule,
}

pub async fn get_candy_task() -> AppResult<CandyTask> {
    let task_guard = CANDY_TASK
        .get()
        .ok_or(AppError::CustomError("Error: Candy task is None".into()))?
        .read()
        .await;

    Ok((*task_guard).clone())
}

pub async fn update_candy_task_rule(
    db: &DatabaseConnection,
    task_id: i32,
    new_rule: CandyTaskRule,
) -> AppResult<CandyTask> {
    let mut task = Tasks::find_by_id(task_id)
        .one(db)
        .await?
        .ok_or(AppError::CustomError("Task not found".into()))?
        .into_active_model();

    task.rule_json = Set(serde_json::to_string(&new_rule)?);
    task.updated_at = Set(chrono::Utc::now().into());

    task.update(db).await?;

    let mut task_guard = CANDY_TASK
        .get()
        .ok_or(AppError::CustomError(
            "Error: Candy task rule is None".into(),
        ))?
        .write()
        .await;

    task_guard.rule = new_rule;

    Ok((*task_guard).clone())
}

impl Storage {
    pub async fn load_candy_rule(&self, task_name: &str) -> AppResult<()> {
        let task = Tasks::find()
            .filter(tasks::Column::TaskName.eq(task_name))
            .one(self.conn.as_ref())
            .await?
            .ok_or(AppError::UserUnExisted(format!(
                "Task {} has not existed",
                task_name
            )))?;

        let rule: CandyTaskRule = serde_json::from_str(&task.rule_json)?;

        let candy_task = CandyTask {
            name: task.task_name,
            description: task.description,
            rule,
        };

        CANDY_TASK
            .set(RwLock::new(candy_task))
            .map_err(|_| AppError::CustomError("Candy task has already been initialized".into()))?;

        Ok(())
    }

    pub async fn record_user_attempt(&self, user_uid: String, reward: i32) -> AppResult<()> {
        let new_attempt = candy::ActiveModel {
            user_uid: Set(user_uid),
            amount: Set(reward),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };
        Candy::insert(new_attempt).exec(self.conn.as_ref()).await?;
        Ok(())
    }

    pub async fn get_user_attempts(&self, user_id: &str) -> AppResult<i32> {
        let count = Candy::find()
            .filter(candy::Column::UserUid.eq(user_id))
            .filter(
                candy::Column::CreatedAt.gte(chrono::Utc::now().date_naive().and_hms_opt(0, 0, 0)),
            )
            .count(self.conn.as_ref())
            .await?;
        Ok(count as i32)
    }

    pub async fn get_user_candy_count(&self, user_uid: &str) -> AppResult<i64> {
        match Candy::find()
            .filter(candy::Column::UserUid.eq(user_uid))
            .select_only()
            .column_as(candy::Column::Amount.sum(), "total_candies")
            .into_model::<AggregationResult>()
            .one(self.conn.as_ref())
            .await?
        {
            Some(aggr_result) => Ok(aggr_result.total_candies.unwrap_or(0)),
            None => Ok(0),
        }
    }
}
