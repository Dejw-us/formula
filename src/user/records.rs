use bcrypt::{BcryptError, BcryptResult, DEFAULT_COST, bcrypt, hash, verify};
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedPassword(pub String);

impl EncodedPassword {
  pub fn verify(&self, password: &str) -> BcryptResult<bool> {
    verify(password, &self.0)
  }
}

pub struct Password(pub String);

impl Password {
  pub fn encode(self) -> BcryptResult<EncodedPassword> {
    let encoded = hash(self.0, DEFAULT_COST)?;

    Ok(EncodedPassword(encoded))
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NoId;

#[derive(Serialize, Deserialize, Debug)]
pub struct User<Id = RecordId> {
  id: Id,
  pub username: String,
  pub password: EncodedPassword,
}

impl User<NoId> {
  pub fn new(username: String, password: EncodedPassword) -> Self {
    Self {
      id: NoId,
      username,
      password,
    }
  }
}

impl User<RecordId> {
  pub fn id(&self) -> &RecordId {
    &self.id
  }
}
