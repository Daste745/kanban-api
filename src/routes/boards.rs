use actix_web::{
    delete, get, patch, post,
    web::{Data, Json, Path, ServiceConfig},
    Error, HttpResponse,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    errors::ServiceError,
    get_conn,
    models::{Board, BoardUpdate, User},
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
    let board = Board::new(&user, data.name, data.description);
    board.save(&pool)?;

    Ok(HttpResponse::Created()
        .header("Location", format!("/{}", board.id))
        .json(board))
}

#[get("/me")]
async fn my_boards(pool: Data<DbPool>, user: User) -> Result<HttpResponse, Error> {
    let conn = get_conn(&pool)?;

    let boards = Board::belonging_to(&user)
        .load::<Board>(&conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(boards))
}

#[get("/{board_id}")]
async fn get_board(pool: Data<DbPool>, Path(board_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    if let Some(board) = Board::find(&pool, board_id)? {
        Ok(HttpResponse::Ok().json(board))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[patch("/{board_id}")]
async fn patch_board(
    pool: Data<DbPool>,
    user: User,
    Path(board_id): Path<Uuid>,
    Json(mut data): Json<BoardUpdate>,
) -> Result<HttpResponse, Error> {
    if data.is_empty() {
        Err(ServiceError::EmptyUpdate)?
    }

    data.id = board_id;

    if let Some(board) = Board::find(&pool, board_id)? {
        if board.owner != user.id {
            Err(HttpResponse::Unauthorized().finish())?
        }

        let board = board.update(&pool, data)?;

        Ok(HttpResponse::Ok().json(board))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[delete("/{board_id}")]
async fn delete_board(
    pool: Data<DbPool>,
    user: User,
    Path(board_id): Path<Uuid>,
) -> Result<HttpResponse, Error> {
    if let Some(board) = Board::find(&pool, board_id)? {
        if board.owner != user.id {
            Err(HttpResponse::Unauthorized().finish())?
        }

        board.delete(&pool)?;
    }

    Ok(HttpResponse::NoContent().finish())
}
