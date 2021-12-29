pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;
use actix_web::web::ServiceConfig;
use diesel::r2d2;

type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn config(cfg: &mut ServiceConfig) {}
