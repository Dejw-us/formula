use std::fmt::Debug;

use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use jsonwebtoken::errors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Serialization,
  BadRequest(Option<String>),
  Server,
  Db,
}

impl Error {
  fn log_error(error: impl Debug) {
    println!("Error: {:?}", error);
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    match self {
      Error::BadRequest(message) => match message {
        Some(message) => (StatusCode::BAD_REQUEST, message).into_response(),
        None => StatusCode::BAD_REQUEST.into_response(),
      },
      _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
  }
}

impl From<BcryptError> for Error {
  fn from(error: BcryptError) -> Self {
    Self::log_error(error);
    Self::Server
  }
}

impl From<surrealdb::Error> for Error {
  fn from(error: surrealdb::Error) -> Self {
    Self::log_error(error);
    Self::Db
  }
}

impl From<serde_json::error::Error> for Error {
  fn from(error: serde_json::error::Error) -> Self {
    Self::log_error(error);
    Self::Serialization
  }
}

impl From<errors::Error> for Error {
  fn from(error: errors::Error) -> Self {
    Self::log_error(error);
    Self::Server
  }
}
