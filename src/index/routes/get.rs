use askama::Template;
use axum::{
  Extension, Json,
  http::{Response, StatusCode, header::CONTENT_TYPE},
  response::IntoResponse,
};

use crate::{
  db::Db,
  index::templates::IndexView,
  user::{dtos::LoginUser, extract::AccessToken},
};

pub async fn index(token: AccessToken) -> Result<impl IntoResponse, StatusCode> {
  println!("Access token: {:?}", token);
  let view = IndexView;
  let body = view
    .render()
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  let res = Response::builder()
    .header(CONTENT_TYPE, "text/html")
    .body(body)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

  Ok(res)
}
