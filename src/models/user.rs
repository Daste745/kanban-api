use actix_web::{dev::Payload, FromRequest, HttpRequest};
use std::{
    future::{ready, Ready},
    str::FromStr,
};

use super::prelude::*;
use crate::{schema::users, Claims};

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
    type Error = ServiceError;
    type Future = Ready<Result<User, Self::Error>>;

    fn from_request(req: &HttpRequest, _pld: &mut Payload) -> Self::Future {
        let claims = match Claims::try_from(req) {
            Ok(c) => c,
            Err(e) => return ready(Err(e)),
        };

        let pool = req.app_data::<Data<DbPool>>().unwrap();

        let user_id = Uuid::from_str(claims.sub.as_str()).unwrap();
        match User::find(&pool, user_id) {
            Ok(u) => match u {
                Some(u) => ready(Ok(u)),
                // FIXME: user could be null if it was deleted after issuing a token
                // TODO: Invalidate tokens that don't belong to any user
                None => ready(Err(ServiceError::InternalServerError)),
            },
            Err(e) => ready(Err(e)),
        }
    }
}

impl User {
    pub fn new(mail: String, password: String) -> Self {
        User {
            id: Uuid::new_v4(),
            mail,
            password,
        }
    }

    pub fn save(&self, pool: &Data<DbPool>) -> Result<usize, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::insert_into(users::table)
            .values(self)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn find(pool: &Data<DbPool>, id: Uuid) -> Result<Option<Self>, ServiceError> {
        let conn = get_conn(pool)?;

        users::table
            .find(id)
            .first::<Self>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn update(&self, pool: &Data<DbPool>, data: UserUpdate) -> Result<Self, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::update(self)
            .set(&data)
            .get_result::<Self>(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn delete(&self, pool: &Data<DbPool>) -> Result<usize, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::delete(self)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }
}

impl UserUpdate {
    /// Returns true if all update fields are None
    pub fn is_empty(&self) -> bool {
        self.mail == None
    }
}
