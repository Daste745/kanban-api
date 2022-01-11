pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;
use actix_web::{
    web::{scope, ServiceConfig},
    Error, HttpRequest,
};
use diesel::r2d2;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};
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
    pub fn from_request(req: &HttpRequest) -> Result<Self, Error> {
        use errors::ServiceError;

        let header = req
            .headers()
            .get("authorization")
            .ok_or(ServiceError::MissingToken)?
            .to_str()
            .map_err(|_| ServiceError::InvalidToken)?;
        let token = header.replace("Bearer ", "");
        // If the token wasn't prefixed with `Bearer ` it will error during validation

        let validation = Validation {
            leeway: 60,
            ..Default::default()
        };
        let token_data = decode::<Claims>(
            &token.as_str(),
            &DecodingKey::from_secret("supersecret".as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            ErrorKind::InvalidToken | ErrorKind::InvalidSignature => ServiceError::InvalidToken,
            ErrorKind::ExpiredSignature => ServiceError::ExpiredToken,
            _ => panic!(),
        });

        Ok(token_data?.claims)
    }
}
