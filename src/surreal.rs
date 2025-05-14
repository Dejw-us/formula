use serde::Deserialize;

use crate::user::records::EncodedPassword;

#[derive(Deserialize, Debug, PartialEq, PartialOrd, Eq)]
pub struct CountQuery {
  pub count: i64,
}

impl CountQuery {
  pub fn is_positive(&self) -> bool {
    self.count > 0
  }
}

#[derive(Deserialize, Debug)]
pub struct PasswordQuery {
  pub password: EncodedPassword,
}
