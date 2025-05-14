use core::arch;
use std::{env, ops::Deref, sync::Arc};

use axum::response::IntoResponse;
use surrealdb::{
  Surreal,
  engine::remote::ws::{Client, Ws},
  opt::auth::Root,
};

#[derive(Clone)]
pub struct Db(Arc<Surreal<Client>>);

impl Deref for Db {
  type Target = Surreal<Client>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Db {
  pub async fn connect() -> surrealdb::Result<Db> {
    let port = env::var("DB_PORT").expect("Provide DB_PORT");
    let addr = format!("localhost:{}", port);
    let username = &env::var("DB_USERNAME").expect("Provide DB_USERNAME");
    let password = &env::var("DB_PASSWORD").expect("Provide DB_PASSWORD");
    let credentials = Root { username, password };

    let client: Surreal<Client> = Surreal::init();

    client.connect::<Ws>(addr).await?;

    client.signin(credentials).await?;

    client.use_ns("formula").use_db("formula").await?;

    Ok(Self(client.into()))
  }
}
