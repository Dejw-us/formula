use axum::{Extension, Router, routing::post};

pub mod dtos;
pub mod records;
pub mod routes;
pub mod service;
pub mod table;

pub fn router() -> Router {
  Router::new()
    .route("/register", post(routes::post::register_user))
    .route("/login", post(routes::post::login_user))
}
