use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal server error")]
    InternalServerError,

    #[display(fmt = "User with this email already exists")]
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

impl ServiceError {
    fn headers(&self) -> Option<Vec<(&str, &str)>> {
        match self {
            ServiceError::MissingToken
            | ServiceError::InvalidToken
            | ServiceError::ExpiredToken
            | ServiceError::InvalidCredentials => Some(vec![("WWW-Authenticate", "Bearer")]),
            _ => None,
        }
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::UserExists => StatusCode::FORBIDDEN,
            ServiceError::MissingToken
            | ServiceError::InvalidToken
            | ServiceError::ExpiredToken
            | ServiceError::InvalidCredentials => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut res = HttpResponse::build(self.status_code());

        if let Some(headers) = self.headers() {
            for (header, value) in headers {
                res.set_header(header, value);
            }
        };

        let msg = format!("{}", self);
        res.json(msg)
    }
}
