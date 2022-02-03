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
    models::{Board, List, ListUpdate, User},
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
    if let Some(board) = Board::find(&pool, board_id)? {
        if board.owner != user.id {
            Err(HttpResponse::Unauthorized().finish())?
        }

        let list = List::new(&board, data.name);
        list.save(&pool)?;

        Ok(HttpResponse::Created()
            .header("Location", format!("/{}", list.id))
            .json(list))
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[get("")]
async fn get_lists(pool: Data<DbPool>, Path(board_id): Path<Uuid>) -> Result<HttpResponse, Error> {
    let conn = get_conn(&pool)?;

    if let Some(board) = Board::find(&pool, board_id)? {
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
    if let Some(_) = Board::find(&pool, board_id)? {
        if let Some(list) = List::find(&pool, list_id)? {
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
    if data.is_empty() {
        Err(ServiceError::EmptyUpdate)?
    }

    if let Some(board) = Board::find(&pool, board_id)? {
        if user.id != board.owner {
            Err(HttpResponse::Unauthorized().finish())?
        }

        if let Some(list) = List::find(&pool, list_id)? {
            if board.id != list.board {
                Err(HttpResponse::Unauthorized().finish())?
            }

            data.id = list_id;

            let list = list.update(&pool, data)?;

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
    if let Some(board) = Board::find(&pool, board_id)? {
        if user.id != board.owner {
            Err(HttpResponse::Unauthorized().finish())?
        }

        if let Some(list) = List::find(&pool, list_id)? {
            if board.id != list.board {
                Err(HttpResponse::BadRequest().finish())?
            }

            list.delete(&pool)?;
        }
    }

    Ok(HttpResponse::NoContent().finish())
}
