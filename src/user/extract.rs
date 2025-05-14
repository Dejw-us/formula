use std::ops::Deref;

use axum::{
  extract::FromRequestParts,
  http::{StatusCode, header::AUTHORIZATION, request::Parts},
};

#[derive(Debug)]
pub struct AccessToken(pub String);

impl Deref for AccessToken {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<S> FromRequestParts<S> for AccessToken {
  type Rejection = StatusCode;

  fn from_request_parts(
    parts: &mut Parts,
    _state: &S,
  ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
    async {
      let header = parts
        .headers
        .get(AUTHORIZATION)
        .ok_or_else(|| StatusCode::UNAUTHORIZED)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
      let token = header.to_str().map_err(|err| {
        println!("Failed to parse header. Error: {:?}", err);
        StatusCode::UNAUTHORIZED
      })?;
      let token = token.strip_prefix("Bearer ").ok_or_else(|| {
        println!("Missing Bearer prefix");
        StatusCode::UNAUTHORIZED
      })?;

      Ok(Self(token.to_string()))
    }
  }
}
