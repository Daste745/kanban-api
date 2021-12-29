use std::{env, error::Error};

use actix_web::{middleware::Logger, App, HttpServer};
use backend::config;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let bind_url = format!("{}:{}", env::var("HOST")?, env::var("PORT")?);

    HttpServer::new(move || App::new().wrap(Logger::default()).configure(config))
        .bind(bind_url)?
        .run()
        .await?;

    Ok(())
}
