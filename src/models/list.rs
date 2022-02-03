use super::prelude::*;
use crate::{models::Board, schema::lists};

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

impl List {
    pub fn new(board: &Board, name: String) -> Self {
        List {
            id: Uuid::new_v4(),
            board: board.id,
            name,
        }
    }

    pub fn save(&self, pool: &Data<DbPool>) -> Result<usize, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::insert_into(lists::table)
            .values(self)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn find(pool: &Data<DbPool>, id: Uuid) -> Result<Option<Self>, ServiceError> {
        let conn = get_conn(pool)?;

        lists::table
            .find(id)
            .first::<Self>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn update(&self, pool: &Data<DbPool>, data: ListUpdate) -> Result<Self, ServiceError> {
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

impl ListUpdate {
    /// Returns true if all update fields are None
    pub fn is_empty(&self) -> bool {
        self.name == None
    }
}
