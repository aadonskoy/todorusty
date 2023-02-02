mod repository;
mod model;
mod api;

use api::task::{get_task, create_task, get_ids};
use actix_web::{HttpServer, App, middleware::Logger};
use repository::reddb::RedisRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        let repo = RedisRepo::init();
        let repo_data = actix_web::web::Data::new(repo);
        let logger = Logger::default();

        App::new()
        .wrap(logger)
        .app_data(repo_data)
        .service(get_task)
        .service(get_ids)
        .service(create_task)
    })
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}
