use super::prelude::*;
use crate::{models::user::User, schema::boards};

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
