use std::{env, io};

use actix_web::{middleware::Logger, App, HttpServer};
use backend::{routes::config, JWTConfig};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let bind_url = format!(
        "{}:{}",
        env::var("HOST").expect("HOST must be set"),
        env::var("PORT").expect("PORT must be set")
    );

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Could not create database connection pool");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_expiry = env::var("JWT_EXPIRY").expect("JWT_EXPIRY must be set");
    let jwt_config = JWTConfig::new(jwt_secret, jwt_expiry);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(jwt_config.clone())
            .configure(config)
    })
    .bind(bind_url)?
    .run()
    .await
}
