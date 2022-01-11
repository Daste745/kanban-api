use actix_web::{
    post,
    web::{Data, Form, ServiceConfig},
    Error, HttpResponse,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use diesel::{prelude::*, result::Error as DieselError};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use uuid::Uuid;

use crate::{errors::ServiceError, schema::users, Claims, DbPool};

pub fn config(cfg: &mut ServiceConfig) {
    // TODO: Allow only POST
    cfg.service(login);
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(pool: Data<DbPool>, Form(data): Form<LoginForm>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let (user_id, password_hash) = users::table
        .filter(users::mail.eq(data.username))
        .select((users::id, users::password))
        .first::<(Uuid, String)>(&conn)
        .map_err(|e| match e {
            DieselError::NotFound => ServiceError::InvalidCredentials,
            _ => ServiceError::InternalServerError,
        })?;

    let password_hash = PasswordHash::new(&password_hash).unwrap();

    Argon2::default()
        .verify_password(data.password.as_bytes(), &password_hash)
        .map_err(|_| ServiceError::InvalidCredentials)?;

    let claims = Claims::new(user_id.to_string());

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("supersecret".as_bytes()),
    )
    .map_err(|_| ServiceError::InternalServerError)?;

    // TODO: Use a strong secret

    Ok(HttpResponse::Ok().json(token))
}

// TODO: POST /logout
