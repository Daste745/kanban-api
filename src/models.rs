use std::{
    future::{ready, Ready},
    str::FromStr,
};

use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    schema::{boards, cards, lists, users},
    Claims, DbPool,
};

#[derive(Debug, Identifiable, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub mail: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UserUpdate {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub mail: Option<String>,
}

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<User, Error>>;

    fn from_request(req: &HttpRequest, _pld: &mut Payload) -> Self::Future {
        let claims = match Claims::try_from(req) {
            Ok(c) => c,
            Err(e) => return ready(Err(e)),
        };

        let pool = req.app_data::<Data<DbPool>>().unwrap();
        let conn = pool.get().unwrap();

        let user_id = Uuid::from_str(claims.sub.as_str()).unwrap();
        // FIXME: user could be null if it was deleted after issuing a token
        let user = users::table.find(user_id).first::<User>(&conn).unwrap();

        // TODO: Invalidate tokens that don't belong to any user

        ready(Ok(user))
    }
}

#[derive(Debug, Identifiable, Queryable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(User, foreign_key = "owner")]
#[table_name = "boards"]
pub struct Board {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[serde(skip_deserializing)]
    pub owner: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "boards"]
pub struct BoardUpdate {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Identifiable, Queryable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(Board, foreign_key = "board")]
#[table_name = "lists"]
pub struct List {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[serde(skip_deserializing)]
    pub board: Uuid,
    pub name: String,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "lists"]
pub struct ListUpdate {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub name: Option<String>,
}

#[derive(Debug, Identifiable, Queryable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(List, foreign_key = "list")]
#[table_name = "cards"]
pub struct Card {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[serde(skip_deserializing)]
    pub list: Uuid,
}
