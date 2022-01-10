pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;
use actix_web::{
    web::{scope, ServiceConfig},
    HttpRequest,
};
use diesel::r2d2;
use jsonwebtoken::{decode, DecodingKey, Validation};
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

impl Claims {
    pub fn from_request(req: &HttpRequest) -> Result<Self, ()> {
        let token = req
            .headers()
            .get("authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .replace("Bearer ", "");

        let validation = Validation {
            leeway: 60,
            ..Default::default()
        };
        let token_data = decode::<Claims>(
            &token.as_str(),
            &DecodingKey::from_secret("supersecret".as_bytes()),
            &validation,
        )
        .unwrap();

        // TODO: Validation

        // TODO: Error handling

        Ok(token_data.claims)
    }
}
