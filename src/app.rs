use std::error::Error;

use axum::{Extension, Router};

use crate::{db::Db, index, user};

pub async fn app() -> Result<Router, Box<dyn Error>> {
  let db = Db::connect().await?;

  let app = Router::new()
    .merge(index::router())
    .merge(user::router())
    .layer(Extension(db));

  Ok(app)
}
