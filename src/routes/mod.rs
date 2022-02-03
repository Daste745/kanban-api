use actix_web::web::{scope, ServiceConfig};

mod auth;
mod boards;
mod cards;
mod lists;
mod users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/users").configure(users::config))
        .service(
            scope("/boards")
                .service(
                    scope("/{board_id}/lists")
                        .service(scope("/{list_id}/cards").configure(cards::config))
                        .configure(lists::config),
                )
                .configure(boards::config),
        );
}
