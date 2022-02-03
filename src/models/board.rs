use super::prelude::*;
use crate::{models::User, schema::boards};

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

impl Board {
    pub fn new(owner: &User, name: String, desc: Option<String>) -> Self {
        Board {
            id: Uuid::new_v4(),
            owner: owner.id,
            name,
            description: desc,
        }
    }

    pub fn save(&self, pool: &Data<DbPool>) -> Result<usize, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::insert_into(boards::table)
            .values(self)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn find(pool: &Data<DbPool>, id: Uuid) -> Result<Option<Self>, ServiceError> {
        let conn = get_conn(pool)?;

        boards::table
            .find(id)
            .first::<Self>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn update(&self, pool: &Data<DbPool>, data: BoardUpdate) -> Result<Self, ServiceError> {
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

impl BoardUpdate {
    /// Returns true if all update fields are None
    pub fn is_empty(&self) -> bool {
        self.name == None && self.description == None
    }
}
