use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse, Responder,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::prelude::*;
use rand_core::OsRng;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    get_conn,
    models::{User, UserUpdate},
    schema::users,
    DbPool,
};

pub fn config(cfg: &mut ServiceConfig) {
    // TODO: Allow only GET, POST, PATCH, DELETE
    cfg.service(get_me)
        .service(patch_me)
        .service(delete_me)
        .service(get_user)
        .service(new_user);
}

#[get("/{user_id}")]
async fn get_user(pool: Data<DbPool>, Path(user_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    if let Some(user) = User::find(&pool, user_id)? {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[post("")]
async fn new_user(pool: Data<DbPool>, Json(data): Json<User>) -> Result<HttpResponse, Error> {
    let conn = get_conn(&pool)?;

    let count = users::table
        .filter(users::mail.eq(data.mail.clone()))
        .count()
        .get_result::<i64>(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    if count > 0 {
        Err(ServiceError::UserExists)?
    }

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let user = User::new(data.mail, password_hash);
    user.save(&pool)?;

    Ok(HttpResponse::Created()
        .header("Location", format!("/{}", user.id))
        .json(user))
}

#[get("/me")]
async fn get_me(user: User) -> impl Responder {
    Json(user)
}

#[patch("/me")]
async fn patch_me(
    pool: Data<DbPool>,
    user: User,
    Json(mut data): Json<UserUpdate>,
) -> Result<HttpResponse, Error> {
    if data.is_empty() {
        Err(ServiceError::EmptyUpdate)?
    }

    data.id = user.id;

    let user = user.update(&pool, data)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/me")]
async fn delete_me(pool: Data<DbPool>, user: User) -> Result<HttpResponse, Error> {
    user.delete(&pool)?;

    // TODO: Invalidate auth token

    Ok(HttpResponse::NoContent().finish())
}
