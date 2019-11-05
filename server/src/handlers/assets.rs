use rocket::{get, http::ContentType, response::content::Html, Response};
use std::io::Cursor;

#[get("/assets/pwa/index.html")]
pub fn pwa_index() -> Response<'static> {
  let mut response = Response::new();
  response.set_header(ContentType::HTML);
  response.set_sized_body(Cursor::new(include_str!("../../../pwa/index.html")));
  response
}

#[get("/assets/pwa/app.js")]
pub fn pwa_app() -> Response<'static> {
  let mut response = Response::new();
  response.set_header(ContentType::JavaScript);
  response.set_sized_body(Cursor::new(include_str!("../../../pwa/pkg/app.js")));
  response
}

#[get("/assets/pwa/app_bg.wasm")]
pub fn pwa_wasm() -> Response<'static> {
  let mut response = Response::new();
  response.set_header(ContentType::WASM);
  let b: &'static [u8] = include_bytes!("../../../pwa/pkg/index_bg.wasm");
  response.set_sized_body(Cursor::new(b));
  response
}

#[get("/assets/pwa/manifest.json")]
pub fn pwa_manifest() -> Response<'static> {
  let mut response = Response::new();
  response.set_header(ContentType::JSON);
  response.set_sized_body(Cursor::new(include_str!("../../../pwa/manifest.json")));
  response
}

#[get("/assets/pwa/sw.js")]
pub fn pwa_sw() -> Response<'static> {
  let mut response = Response::new();
  response.set_header(ContentType::JavaScript);
  response.set_sized_body(Cursor::new(include_str!("../../../pwa/sw.js")));
  response
}
