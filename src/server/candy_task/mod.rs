mod candy_task_handler;
mod candy_task_router;

pub use candy_task_router::candy_router;

use crate::app::SharedState;
use crate::common::error::AppResult;

pub async fn init(state: SharedState) -> AppResult<()> {
    state.store.load_candy_rule("candy".into()).await?;

    Ok(())
}
