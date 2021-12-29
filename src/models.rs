use crate::schema::users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    #[serde(skip_deserializing)]
    pub id: Uuid,
    pub mail: String,
    #[serde(skip_serializing)]
    pub password: String,
}
