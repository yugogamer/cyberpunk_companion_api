use actix_web::{error::HttpError, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("email or username already in use")]
    EmailAlreadyUsed(),
    #[error("data store disconnected")]
    NoUserFinde,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AppErrorsResponse {
    pub status_code: u16,
    pub types: String,
    pub message: String,
}

impl AppErrorsResponse {
    pub fn new(error: &AppErrors) -> Self {
        let types = format!("{:?}", error);
        let message = error.to_string();
        match error {
            AppErrors::EmailAlreadyUsed() => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::NoUserFinde => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::InvalidToken => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::InvalidPassword(_) => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::DecoderError(_) => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::InvalidJwt(_) => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::InvalideBase64(_) => Self {
                status_code: 400,
                types,
                message,
            },
            AppErrors::ConnectionErrors(_) => Self {
                status_code: 500,
                types,
                message,
            },
            AppErrors::MigrateErrors(_) => Self {
                status_code: 500,
                types,
                message,
            },
            AppErrors::InvalidSecretLenght(_) => Self {
                status_code: 500,
                types,
                message,
            },
            AppErrors::HttpError(_) => Self {
                status_code: 500,
                types,
                message,
            },
        }
    }
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let response = AppErrorsResponse::new(&self);
        HttpResponse::build(actix_web::http::StatusCode::from_u16(response.status_code).unwrap())
            .json(response)
    }
}
