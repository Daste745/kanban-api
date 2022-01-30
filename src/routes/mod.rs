use actix_web::web::{scope, ServiceConfig};

mod auth;
mod users;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(auth::config))
        .service(scope("/users").configure(users::config));
}
