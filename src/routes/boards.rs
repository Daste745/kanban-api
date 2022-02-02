use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    models::{Board, BoardUpdate, User},
    schema::boards,
    DbPool,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(my_boards)
        .service(get_board)
        .service(new_board)
        .service(patch_board)
        .service(delete_board);
}

#[post("")]
async fn new_board(
    pool: Data<DbPool>,
    user: User,
    Json(data): Json<Board>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = Board {
        id: Uuid::new_v4(),
        owner: user.id,
        name: data.name,
        description: data.description,
    };

    diesel::insert_into(boards::table)
        .values(&board)
        .execute(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Created()
        .header("Location", format!("/{}", board.id))
        .json(board))
}

#[get("/me")]
async fn my_boards(pool: Data<DbPool>, user: User) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let boards = Board::belonging_to(&user)
        .load::<Board>(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(boards))
}

#[get("/{board_id}")]
async fn get_board(pool: Data<DbPool>, Path(board_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(if let Some(board) = board {
        HttpResponse::Ok().json(board)
    } else {
        HttpResponse::NotFound().finish()
    })
}

#[patch("/{board_id}")]
async fn patch_board(
    pool: Data<DbPool>,
    _user: User,
    Path(board_id): Path<Uuid>,
    Json(mut data): Json<BoardUpdate>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    if data.name == None && data.description == None {
        Err(ServiceError::EmptyUpdate)?
    }

    data.id = board_id;

    let board = boards::table
        .find(board_id)
        .first::<Board>(&conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(board) = board {
        let board = diesel::update(&board)
            .set(&data)
            .get_result::<Board>(&conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        Ok(HttpResponse::Ok().json(board))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[delete("/{board_id}")]
async fn delete_board(
    pool: Data<DbPool>,
    _user: User,
    Path(board_id): Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().unwrap();

    diesel::delete(boards::table.find(board_id))
        .execute(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
