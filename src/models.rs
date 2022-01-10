use std::{
    future::{ready, Ready},
    str::FromStr,
};

use actix_web::{dev::Payload, web::Data, Error, FromRequest, HttpRequest};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{schema::users, Claims, DbPool};

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub mail: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<User, Error>>;

    fn from_request(req: &HttpRequest, _pld: &mut Payload) -> Self::Future {
        let claims = Claims::from_request(req).unwrap();

        // TODO: Verify claims

        let pool = req.app_data::<Data<DbPool>>().unwrap();
        let conn = pool.get().unwrap();

        let user_id = Uuid::from_str(claims.sub.as_str()).unwrap();
        let user = users::table.find(user_id).first::<User>(&conn).unwrap();

        // TODO: Error handling

        ready(Ok(user))
    }
}
