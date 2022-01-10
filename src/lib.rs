pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;
use actix_web::web::{scope, ServiceConfig};
use diesel::r2d2;
use serde::{Deserialize, Serialize};

use routes::{auth, users};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/users").configure(users::config))
        .service(scope("/auth").configure(auth::config));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}
