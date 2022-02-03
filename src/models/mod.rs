mod board;
mod card;
mod list;
mod user;

pub use board::{Board, BoardUpdate};
pub use card::Card;
pub use list::{List, ListUpdate};
pub use user::{User, UserUpdate};

/// Global uses that are neccessary in *almost every* model definition
mod prelude {
    pub use actix_web::web::Data;
    pub use diesel::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use uuid::Uuid;

    pub use crate::{errors::ServiceError, get_conn, DbPool};
}
