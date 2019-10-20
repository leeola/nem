use {
  crate::acme,
  rocket::{
    response::{self, Responder},
    Request, Response,
  },
  std::{io::Cursor, path::PathBuf},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Internal(String),
}

impl<'r> Responder<'r> for Error {
  fn respond_to(self, _: &Request) -> response::Result<'r> {
    Response::build()
      .sized_body(Cursor::new("error handling not implemented"))
      .ok()
  }
}

#[derive(Debug)]
pub enum InitError {
  /// A Server was called with an Acme TLS config, or vice versa.
  InvalidServerTlsVariant,
  Acme(acme::Error),
  InvalidAcmePath(PathBuf),
  RocketConfig(rocket::config::ConfigError),
}

impl From<acme::Error> for InitError {
  fn from(err: acme::Error) -> Self {
    Self::Acme(err)
  }
}
