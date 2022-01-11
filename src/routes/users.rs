use actix_web::{
    get, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse, Responder,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{insert_into, prelude::*};
use rand_core::OsRng;
use uuid::Uuid;

use crate::{models::User, schema::users, DbPool};

pub fn config(cfg: &mut ServiceConfig) {
    // TODO: Allow only GET, POST, PATCH, DELETE
    cfg.service(get_me).service(get_user).service(new_user);
}

#[get("/{user_id}")]
async fn get_user(pool: Data<DbPool>, Path(user_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let user = users::table
        .find(user_id)
        .first::<User>(&conn)
        .optional()
        .map_err(|_| HttpResponse::InternalServerError().json("Internal server error"))?;

    // TODO: Rework this as a Responder

    Ok(if let Some(user) = user {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    })
}

#[post("")]
async fn new_user(pool: Data<DbPool>, Json(data): Json<User>) -> impl Responder {
    let conn = pool.get().unwrap();

    // TODO: Ensure unique mail

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User {
        id: Uuid::new_v4(),
        mail: data.mail,
        password: password_hash,
    };

    insert_into(users::table)
        .values(&user)
        .execute(&conn)
        .unwrap();

    Json(user)
}

#[get("/me")]
async fn get_me(user: User) -> impl Responder {
    Json(user)
}

// TODO: Protected endpoints: PATCH and DELETE
