use {
  rocket::{
    response::{self, Responder},
    Request, Response,
  },
  std::io::Cursor,
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
  PathNotValidString,
  RocketConfig(rocket::config::ConfigError),
}
