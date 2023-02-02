use crate::repository::reddb::{RedisRepo};
use crate::model::task::{Task, TasksList};

use actix_web::{get, post, web, Responder, Result};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct TaskIdentifier {
    pub task_id: String,
}

#[derive(Deserialize)]
pub struct SubmitTaskRequest {
    description: String,
}

#[get("/tasks")]
pub async fn get_ids(repo: web::Data<RedisRepo>) -> Result<impl Responder> {
    let tasks: TasksList = repo.get_all().unwrap();

    Ok(web::Json(tasks))
}

#[get("/task/{task_id}")]
pub async fn get_task(repo: web::Data<RedisRepo>, task_identifier: web::Path<TaskIdentifier>) -> Result<impl Responder> {
    let task_id = task_identifier.into_inner().task_id;
    let task = repo.get(task_id.clone()).unwrap();

    Ok(web::Json(task))
}

#[post("/task")]
pub async fn create_task(repo: web::Data<RedisRepo>, request: web::Json<SubmitTaskRequest>) -> Result<impl Responder> {
    let task = Task::new(request.into_inner().description.clone());
    let res_task = repo.create(task).unwrap();

    Ok(web::Json(res_task))
}