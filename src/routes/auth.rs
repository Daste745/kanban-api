use actix_web::{
    post,
    web::{Data, Form, ServiceConfig},
    Responder,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{self, Duration};
use diesel::prelude::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use uuid::Uuid;

use crate::{schema::users, Claims, DbPool};

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
async fn login(pool: Data<DbPool>, Form(data): Form<LoginForm>) -> impl Responder {
    let conn = pool.get().unwrap();

    let (user_id, password_hash) = users::table
        .filter(users::mail.eq(data.username))
        .select((users::id, users::password))
        .first::<(Uuid, String)>(&conn)
        .unwrap();

    let password_hash = PasswordHash::new(&password_hash).unwrap();

    if Argon2::default()
        .verify_password(data.password.as_bytes(), &password_hash)
        .is_err()
    {
        // TODO: `WWW-Authenticate: Bearer` on any 401 UNAUTHORIZED response

        // TODO: Send an error
        return String::from("Wrong credentials");
    }

    let now = chrono::Utc::now();
    // TODO: Configurable expiry time
    let exp = now + Duration::hours(1);

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("supersecret".as_bytes()),
    );

    // TODO: Use a strong secret

    // TODO: Error handling

    token.unwrap()
}

// TODO: POST /logout
