use crate::{
    common::error::{AppError, AppResult},
    database::{
        entities::{prelude::Tasks, tasks},
        Storage,
    },
};
use sea_orm::*;

impl Storage {
    pub async fn get_all_tasks(&self) -> AppResult<Vec<tasks::Model>> {
        let tasks = Tasks::find().all(self.conn.as_ref()).await?;
        Ok(tasks)
    }

    pub async fn add_task(&self, task_name: &str, description: &str, points: i32) -> AppResult<()> {
        let new_task = tasks::ActiveModel {
            id: NotSet,
            task_name: Set(task_name.to_owned()),
            description: Set(description.to_owned()),
            points: Set(points),
            created_at: Set(chrono::Utc::now().into()),
        };

        new_task.insert(self.conn.as_ref()).await?;
        Ok(())
    }
}
