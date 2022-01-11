use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Missing access token")]
    MissingToken,

    #[display(fmt = "Invalid access token")]
    InvalidToken,

    #[display(fmt = "Expired access token")]
    ExpiredToken,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ServiceError::MissingToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Missing access token"),

            ServiceError::InvalidToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Invalid access token"),

            ServiceError::ExpiredToken => HttpResponse::Unauthorized()
                .header("WWW-Authenticate", "Bearer")
                .json("Expired access token"),
        }
    }
}
