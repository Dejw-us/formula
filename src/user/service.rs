use std::{env, sync::LazyLock};

use chrono::{Duration, Local};
use jsonwebtoken::{EncodingKey, Header, TokenData, encode, errors};
use serde::{Deserialize, Serialize};

use crate::{db::Db, surreal::CountQuery};

use super::table::USERS;

const JWT_SECRET: LazyLock<String> =
  LazyLock::new(|| env::var("JWT_SECRET").expect("Provide JWT_SECRET"));

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
  pub sub: String,
  pub exp: i64,
}

pub async fn user_exists(db: &Db, username: &str) -> surrealdb::Result<bool> {
  let query = format!("SELECT COUNT() FROM user WHERE username='{}'", username);
  let mut res = db.query(query).await?;
  let count: Option<CountQuery> = res.take(0)?;
  let exists = count.map(|c| c.is_positive()).unwrap_or(false);
  Ok(exists)
}

pub fn generate_token(username: &str) -> errors::Result<String> {
  let exp = Local::now()
    .checked_sub_signed(Duration::minutes(15))
    .expect("Invalid timestamp")
    .timestamp();
  let claims = Claims {
    sub: username.to_string(),
    exp,
  };
  let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(JWT_SECRET.as_ref()),
  )?;

  Ok(token)
}
