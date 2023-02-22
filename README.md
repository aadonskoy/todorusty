# Todo app
Simple web api for creating lists.
Made for educational purposes using actix and redis.

## How to run

Start redis:
`docker compose up -d`

Start application
`cargo run`

## How to use:

 - create task
```
 [POST] localhost:8088/task
 {
  "description": "some description"
 }
```
 - list of tasks
```
 [GET] localhost:8088/tasks
```
 - task by id
```
 [GET] localhost:8088/task/task-id
 ```
 _note:_ "not found" was not implemented yet