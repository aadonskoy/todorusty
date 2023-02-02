use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub task_id: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct TasksList {
    pub items: Vec<Task>
}

impl Task {
    pub fn new(description: String) -> Self {
        Self {
            task_id: Uuid::new_v4().to_string(),
            description
        }
    }
}