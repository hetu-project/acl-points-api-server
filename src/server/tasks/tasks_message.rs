use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub rule: String,
    pub desc: String,
}
