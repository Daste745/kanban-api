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
}
