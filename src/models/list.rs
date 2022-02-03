use super::prelude::*;
use crate::{models::board::Board, schema::lists};

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
