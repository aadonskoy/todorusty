extern crate redis;

use crate::model::task::{Task, TasksList};

pub struct RedisRepo {
    client: redis::Client,
    index_name: String,
}

impl RedisRepo {
    pub fn init() -> RedisRepo {
        let client = redis::Client::open("redis://127.0.0.1:6380").unwrap();
        RedisRepo { client, index_name: "tasks_index".to_string() }
    }

    pub fn create(&self, task: Task) -> redis::RedisResult<Task> {
        let mut conn = self.client.get_connection()?;
        redis::cmd("SADD").arg(&self.index_name).arg(&task.task_id).query(&mut conn)?;
        redis::cmd("SET").arg(&task.task_id).arg(&task.description).query(&mut conn)?;
        Ok(task)
    }

    pub fn get(&self, task_id: String) -> redis::RedisResult<Task> {
        let mut conn = self.client.get_connection()?;
        let res = redis::cmd("GET").arg(&task_id).query::<String>(&mut conn)?;
        Ok(Task { task_id, description: res })
    }

    pub fn get_all(&self) -> redis::RedisResult<TasksList> {
        let mut conn = self.client.get_connection()?;
        let ids = redis::cmd("SMEMBERS").arg(&self.index_name).query::<Vec<String>>(&mut conn)?;

        let tasks = ids.into_iter()
            .map(|id| Task { task_id: id.clone(), description: redis::cmd("GET").arg(&id).query::<String>(&mut conn).unwrap() })
            .collect();
        Ok(TasksList { items: tasks })
    }
}