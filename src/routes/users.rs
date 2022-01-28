use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse, Responder,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use diesel::{insert_into, prelude::*};
use rand_core::OsRng;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
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
    let conn = pool.get().unwrap();

    let user = users::table
        .find(user_id)
        .first::<User>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(if let Some(user) = user {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    })
}

#[post("")]
async fn new_user(pool: Data<DbPool>, Json(data): Json<User>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

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

    let user = User {
        id: Uuid::new_v4(),
        mail: data.mail,
        password: password_hash,
    };

    insert_into(users::table)
        .values(&user)
        .execute(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

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
    let conn = pool.get().unwrap();

    if data.mail == None {
        Err(ServiceError::EmptyUpdate)?
    }

    data.id = user.id;

    let user = diesel::update(&user)
        .set(&data)
        .get_result::<User>(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/me")]
async fn delete_me(pool: Data<DbPool>, user: User) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    diesel::delete(users::table.find(user.id))
        .execute(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    // TODO: Invalidate auth token

    Ok(HttpResponse::NoContent().finish())
}
