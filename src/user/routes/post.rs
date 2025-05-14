use std::time::Instant;

use axum::{
  Extension, Json,
  http::{Response, StatusCode, header::CONTENT_TYPE},
  response::IntoResponse,
};
use surrealdb::{Value, method::Query};

use crate::{
  db::Db,
  error::{Error, Result},
  surreal::PasswordQuery,
  user::{
    dtos::{LoginUser, RegisterUser, Tokens},
    records::{NoId, User},
    service::{self, generate_token},
    table::USERS,
  },
};

pub async fn login_user(db: Extension<Db>, body: Json<LoginUser>) -> Result<Response<String>> {
  let query = format!("SELECT * FROM user WHERE username='{}'", body.username);
  let mut res = db.query(query).await?;
  let password: Option<PasswordQuery> = res.take(0)?;

  match password {
    Some(query) => {
      if query.password.verify(&body.password)? {
        let token = generate_token(&body.username)?;
        let body = Tokens {
          access_token: token,
        };
        Ok(
          Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&body)?)
            .unwrap(),
        )
      } else {
        Ok(
          Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Bad credentials".to_string())
            .unwrap(),
        )
      }
    }
    None => Err(Error::BadRequest(Some("User does not exist".to_string()))),
  }
}

pub async fn register_user(
  db: Extension<Db>,
  body: Json<RegisterUser>,
) -> Result<Response<String>> {
  if service::user_exists(&db, &body.username).await? {
    return Err(Error::BadRequest(Some("User already exists".to_string())));
  }
  if !body.passwords_match() {
    return Err(Error::BadRequest(Some(
      "Passwords do not match".to_string(),
    )));
  }

  let user: User<NoId> = body.0.try_into()?;
  let user: Option<User> = db.create(USERS).content(user).await?;

  match user {
    Some(user) => Ok(
      Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(serde_json::to_string(&user)?)
        .unwrap(),
    ),
    None => Err(Error::Server),
  }
}
