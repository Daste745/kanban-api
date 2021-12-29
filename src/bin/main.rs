use std::{env, error::Error};

use actix_web::{middleware::Logger, App, HttpServer};
use backend::config;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    env_logger::init();

    let bind_url = format!("{}:{}", env::var("HOST")?, env::var("PORT")?);

    let database_url = env::var("DATABASE_URL")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .configure(config)
    })
    .bind(bind_url)?
    .run()
    .await?;

    Ok(())
}
