pub mod errors;
pub mod models;
pub mod routes;
pub mod schema;

#[macro_use]
extern crate diesel;

use actix_web::{web::Data, Error, HttpRequest};
use chrono::{self, Duration};
use diesel::r2d2;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};
use parse_duration::parse;
use serde::{Deserialize, Serialize};

type DbPool = r2d2::Pool<r2d2::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Clone)]
pub struct JWTConfig {
    key: String,
    expiry: Duration,
}

impl JWTConfig {
    pub fn new(key: String, expiry: String) -> Self {
        let expiry = parse(expiry.as_str()).expect("JWT_EXPIRY must be a valid duration");
        let expiry = Duration::from_std(expiry).unwrap();

        Self { key, expiry }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

impl Claims {
    pub fn new(sub: String, expiry: Duration) -> Self {
        let now = chrono::Utc::now();
        let exp = now + expiry;

        Self {
            sub,
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        }
    }
}

impl TryFrom<&HttpRequest> for Claims {
    type Error = Error;

    fn try_from(req: &HttpRequest) -> Result<Self, Error> {
        use errors::ServiceError;

        let header = req
            .headers()
            .get("authorization")
            .ok_or(ServiceError::MissingToken)?
            .to_str()
            .map_err(|_| ServiceError::InvalidToken)?;
        let token = header.replace("Bearer ", "");
        // If the token wasn't prefixed with `Bearer ` it will error during validation

        let jwt_config = req.app_data::<Data<JWTConfig>>().unwrap();
        let validation = Validation {
            leeway: 60,
            ..Default::default()
        };

        let token_data = decode::<Claims>(
            &token.as_str(),
            &DecodingKey::from_secret(jwt_config.key.as_bytes()),
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
