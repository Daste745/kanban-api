use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal server error")]
    InternalServerError,

    #[display(fmt = "User already exists")]
    UserExists,

    #[display(fmt = "Missing access token")]
    MissingToken,

    #[display(fmt = "Invalid access token")]
    InvalidToken,

    #[display(fmt = "Expired access token")]
    ExpiredToken,

    #[display(fmt = "Invalid user credentials")]
    InvalidCredentials,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError().finish(),

            ServiceError::UserExists => {
                HttpResponse::Forbidden().json("User with this email already exists")
            }

            ServiceError::MissingToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Missing access token"),

            ServiceError::InvalidToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Invalid access token"),

            ServiceError::ExpiredToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Expired access token"),

            ServiceError::InvalidCredentials => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Invalid login credentials"),
        }
    }
}
