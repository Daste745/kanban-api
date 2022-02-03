use super::prelude::*;
use crate::{models::list::List, schema::cards};

#[derive(Debug, Identifiable, Queryable, Insertable, Associations, Serialize, Deserialize)]
#[belongs_to(List, foreign_key = "list")]
#[table_name = "cards"]
pub struct Card {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    #[serde(skip_deserializing)]
    pub list: Uuid,
    pub content: Option<String>,
    pub labels: Option<Vec<String>>,
}

#[derive(Debug, AsChangeset, Deserialize)]
#[table_name = "cards"]
pub struct CardUpdate {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub content: Option<String>,
    pub labels: Option<Vec<String>>,
}

impl Card {
    pub fn new(list: &List, content: Option<String>, labels: Option<Vec<String>>) -> Self {
        Card {
            id: Uuid::new_v4(),
            list: list.id,
            content,
            labels,
        }
    }

    pub fn save(&self, pool: &Data<DbPool>) -> Result<usize, ServiceError> {
        let conn = get_conn(pool)?;

        diesel::insert_into(cards::table)
            .values(self)
            .execute(&conn)
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn find(pool: &Data<DbPool>, id: Uuid) -> Result<Option<Self>, ServiceError> {
        let conn = get_conn(pool)?;

        cards::table
            .find(id)
            .first::<Self>(&conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)
    }

    pub fn update(&self, pool: &Data<DbPool>, data: CardUpdate) -> Result<Self, ServiceError> {
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

impl CardUpdate {
    /// Returns true if all update fields are None
    pub fn is_empty(&self) -> bool {
        self.content == None && self.labels == None
    }
}
