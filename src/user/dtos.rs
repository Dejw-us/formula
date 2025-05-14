use bcrypt::BcryptError;
use serde::Deserialize;

use crate::error::Error;

use super::records::{NoId, Password, User};

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
  pub username: String,
  pub password: String,
  pub repeated_password: String,
}

impl RegisterUser {
  pub fn passwords_match(&self) -> bool {
    self.password == self.repeated_password
  }
}

impl TryInto<User<NoId>> for RegisterUser {
  type Error = BcryptError;

  fn try_into(self) -> Result<User<NoId>, Self::Error> {
    Ok(User::new(self.username, Password(self.password).encode()?))
  }
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
  pub username: String,
  pub password: String,
}
