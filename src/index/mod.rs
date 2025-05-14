use axum::{Router, routing::get};

pub mod routes;
pub mod templates;

pub fn router() -> Router {
  Router::new().route("/", get(routes::get::index))
}
