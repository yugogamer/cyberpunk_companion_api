use actix_web::{dev::ServiceRequest, FromRequest, HttpResponse, ResponseError};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::{error::Error, future::Ready};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("data store disconnected")]
    NoUserFinded,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            AuthError::NoUserFinded => HttpResponse::Unauthorized().body("No user finded"),
        }
    }
}

pub struct User;

impl FromRequest for User {
    type Error = AuthError;
    type Future = Ready<Result<User, AuthError>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        todo!()
    }
}
