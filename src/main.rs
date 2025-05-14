use std::{env, error::Error};

use app::app;
use tokio::net::TcpListener;

mod app;
mod db;
mod error;
mod form;
mod index;
mod surreal;
mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  #[cfg(debug_assertions)]
  dotenv::dotenv().expect("Failed to load .env file");

  let port = env::var("SERVER_PORT").expect("Specify SERVER_PORT");
  let addr = format!("0.0.0.0:{}", port);
  let listener = TcpListener::bind(addr).await?;

  axum::serve(listener, app().await?).await?;

  Ok(())
}
