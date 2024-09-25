
mod application;
mod presentation;
mod domain;
mod infrastructure;
mod schema;

use env_logger::Env;
use dotenv::dotenv;

use infrastructure::web::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //initialziing the environment loger
    env_logger::Builder
        ::from_env(Env::default().default_filter_or("info"))
        .init();
    //loading the env variables from the env file
    dotenv().ok();
   
    run().await
}