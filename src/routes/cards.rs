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
    models::{Board, Card, CardUpdate, List, User},
    DbPool,
};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(new_card)
        .service(get_cards)
        .service(get_card)
        .service(patch_card)
        .service(delete_card);
}

#[post("")]
async fn new_card(
    pool: Data<DbPool>,
    user: User,
    Path((board_id, list_id)): Path<(Uuid, Uuid)>,
    Json(data): Json<Card>,
) -> Result<HttpResponse, Error> {
    if let Some(board) = Board::find(&pool, board_id)? {
        if board.owner != user.id {
            Err(ServiceError::InvalidCredentials)?
        }

        if let Some(list) = List::find(&pool, list_id)? {
            if list.board != board.id {
                Err(HttpResponse::Unauthorized().finish())?
            }

            let card = Card::new(&list, data.content, data.labels);
            card.save(&pool)?;

            Ok(HttpResponse::Created()
                .header("Location", format!("/{}", card.id))
                .json(card))
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[get("")]
async fn get_cards(
    pool: Data<DbPool>,
    Path((board_id, list_id)): Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let conn = get_conn(&pool)?;

    if let Some(board) = Board::find(&pool, board_id)? {
        if let Some(list) = List::find(&pool, list_id)? {
            if list.board != board.id {
                Err(HttpResponse::NotFound().finish())?
            }

            let cards = Card::belonging_to(&list)
                .load::<Card>(&conn)
                .map_err(|_| ServiceError::InternalServerError)?;

            Ok(HttpResponse::Ok().json(cards))
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[get("/{card_id}")]
async fn get_card(
    pool: Data<DbPool>,
    Path((board_id, list_id, card_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    if let Some(board) = Board::find(&pool, board_id)? {
        if let Some(list) = List::find(&pool, list_id)? {
            if let Some(card) = Card::find(&pool, card_id)? {
                if list.board != board.id || card.list != list.id {
                    Err(HttpResponse::NotFound().finish())?
                }

                Ok(HttpResponse::Ok().json(card))
            } else {
                Err(HttpResponse::NotFound().finish())?
            }
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[patch("/{card_id}")]
async fn patch_card(
    pool: Data<DbPool>,
    user: User,
    Path((board_id, list_id, card_id)): Path<(Uuid, Uuid, Uuid)>,
    Json(mut data): Json<CardUpdate>,
) -> Result<HttpResponse, Error> {
    if data.is_empty() {
        Err(ServiceError::EmptyUpdate)?
    }

    if let Some(board) = Board::find(&pool, board_id)? {
        if let Some(list) = List::find(&pool, list_id)? {
            if let Some(card) = Card::find(&pool, card_id)? {
                if user.id != board.owner || list.board != board.id || card.list != list.id {
                    Err(HttpResponse::Unauthorized().finish())?
                }

                data.id = card_id;

                let card = card.update(&pool, data)?;

                Ok(HttpResponse::Ok().json(card))
            } else {
                Err(HttpResponse::NotFound().finish())?
            }
        } else {
            Err(HttpResponse::NotFound().finish())?
        }
    } else {
        Err(HttpResponse::NotFound().finish())?
    }
}

#[delete("/{card_id}")]
async fn delete_card(
    pool: Data<DbPool>,
    user: User,
    Path((board_id, list_id, card_id)): Path<(Uuid, Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    if let Some(board) = Board::find(&pool, board_id)? {
        if let Some(list) = List::find(&pool, list_id)? {
            if let Some(card) = Card::find(&pool, card_id)? {
                if user.id != board.owner || list.board != board.id || card.list != list.id {
                    Err(HttpResponse::Unauthorized().finish())?
                }

                card.delete(&pool)?;
            }
        }
    }

    Ok(HttpResponse::NoContent().finish())
}
