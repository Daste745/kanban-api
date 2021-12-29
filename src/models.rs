use crate::schema::users;
use uuid::Uuid;

#[derive(Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub mail: String,
    pub password: String,
}
