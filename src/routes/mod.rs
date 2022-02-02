use actix_web::web::{scope, ServiceConfig};

mod auth;
mod boards;
mod lists;
mod users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/users").configure(users::config))
        .service(scope("/boards/{board_id}/lists").configure(lists::config))
        .service(scope("/boards").configure(boards::config));
}
