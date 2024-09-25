use actix_web::App;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::HttpServer;
use log::info;

use crate::presentation::routes;
use super::repositories::postgres_user_repository::PostgresUserRepository;


pub async fn run() -> std::io::Result<()> {
    let repo=PostgresUserRepository::new();
    let app_data=web::Data::new(repo);

    info!("starting..!");

    HttpServer::new(move || {
        App::new()
            .app_data(
                app_data.clone()
            )
            .wrap(Logger::default())
            .configure(routes::user_routes::routes)
    })
        .bind("0.0.0.0:8000")
        .unwrap()
        .run()
        .await
}