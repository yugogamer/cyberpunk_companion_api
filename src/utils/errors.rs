use actix_web::{error::HttpError, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("data store disconnected")]
    NoUserFinded,
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid password")]
    InvalidPassword(#[from] argon2::Error),
    #[error("decode")]
    DecoderError(#[from] base64::DecodeError),
    #[error("invalid token")]
    InvalidJwt(#[from] jwt::Error),
    #[error("invalide base64")]
    InvalideBase64(#[from] std::str::Utf8Error),
    #[error("database connection error")]
    ConnectionErrors(#[from] sqlx::Error),
    #[error("database migration errors")]
    MigrateErrors(#[from] sqlx::migrate::MigrateError),
    #[error("invalid secret lenght")]
    InvalidSecretLenght(#[from] sha2::digest::InvalidLength),
    #[error("http errors")]
    HttpError(#[from] HttpError),
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            AppErrors::NoUserFinded => HttpResponse::Unauthorized().body("No user finded"),
            AppErrors::InvalidToken => HttpResponse::Unauthorized().body("Invalid token"),
            AppErrors::InvalidPassword(_) => {
                HttpResponse::InternalServerError().body("Invalid password")
            }
            AppErrors::DecoderError(_) => HttpResponse::InternalServerError().body("Decode error"),
            AppErrors::InvalidJwt(_) => HttpResponse::InternalServerError().body("Invalid jwt"),
            AppErrors::InvalideBase64(_) => {
                HttpResponse::InternalServerError().body("Invalid base64")
            }
            AppErrors::ConnectionErrors(err) => {
                HttpResponse::InternalServerError().body(format!("err : {:?}", err))
            }
            AppErrors::MigrateErrors(_) => {
                HttpResponse::InternalServerError().body("Database migration error")
            }
            AppErrors::InvalidSecretLenght(_) => {
                HttpResponse::InternalServerError().body("Invalid secret lenght")
            }
            AppErrors::HttpError(_) => HttpResponse::InternalServerError().body("http errors"),
        }
    }
}
