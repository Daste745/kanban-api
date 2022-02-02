use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    models::{Board, List, ListUpdate, User},
    schema::{boards, lists},
    DbPool,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(new_list)
        .service(get_lists)
        .service(get_list)
        .service(patch_list)
        .service(delete_list);
}

#[post("")]
async fn new_list(
    pool: Data<DbPool>,
    user: User,
    Path(board_id): Path<Uuid>,
    Json(data): Json<List>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(board) = board {
        if board.owner != user.id {
            Err(ServiceError::InvalidCredentials)?
        }

        let list = List {
            id: Uuid::new_v4(),
            board: board.id,
            name: data.name,
        };

        diesel::insert_into(lists::table)
            .values(&list)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(HttpResponse::Created()
            .header("Location", format!("/{}", list.id))
            .json(list))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[get("")]
async fn get_lists(pool: Data<DbPool>, Path(board_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(board) = board {
        let lists = List::belonging_to(&board)
            .load::<List>(&conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(HttpResponse::Ok().json(lists))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[get("/{list_id}")]
async fn get_list(
    pool: Data<DbPool>,
    Path((board_id, list_id)): Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(_) = board {
        let list = lists::table
            .find(list_id)
            .first::<List>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)?;

        if let Some(list) = list {
            Ok(HttpResponse::Ok().json(list))
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[patch("/{list_id}")]
async fn patch_list(
    pool: Data<DbPool>,
    user: User,
    Path((board_id, list_id)): Path<(Uuid, Uuid)>,
    Json(mut data): Json<ListUpdate>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    if data.name == None {
        Err(ServiceError::EmptyUpdate)?
    }

    data.id = board_id;

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(board) = board {
        if user.id != board.owner {
            Err(HttpResponse::Unauthorized().finish())?
        }

        let list = List::belonging_to(&board)
            .find(list_id)
            .first::<List>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)?;

        if let Some(list) = list {
            if board.id != list.board {
                Err(HttpResponse::Unauthorized().finish())?
            }

            let list = diesel::update(&list)
                .set(&data)
                .get_result::<List>(&conn)
                .map_err(|_| ServiceError::InternalServerError)?;

            Ok(HttpResponse::Ok().json(list))
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[delete("/{list_id}")]
async fn delete_list(
    pool: Data<DbPool>,
    user: User,
    Path((board_id, list_id)): Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(board) = board {
        if user.id != board.owner {
            Err(HttpResponse::Unauthorized().finish())?
        }

        let list = List::belonging_to(&board)
            .find(list_id)
            .first::<List>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)?;

        if let Some(list) = list {
            if board.id != list.board {
                Err(HttpResponse::BadRequest().finish())?
            }

            diesel::delete(&list)
                .execute(&conn)
                .map_err(|_| ServiceError::InternalServerError)?;
        }
    }

    Ok(HttpResponse::NoContent().finish())
}
