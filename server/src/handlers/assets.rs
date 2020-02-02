use rocket::{get, http::ContentType, Response};
use std::io::Cursor;

#[get("/assets/pwa/index.html")]
pub fn pwa_html() -> Response<'static> {
    let mut response = Response::new();
    response.set_header(ContentType::HTML);
    response.set_sized_body(Cursor::new(include_str!(concat!(
        env!("PWA_DIR"),
        "/index.html"
    ))));
    response
}

#[get("/assets/pwa/index.js")]
pub fn pwa_js() -> Response<'static> {
    let mut response = Response::new();
    response.set_header(ContentType::JavaScript);
    response.set_sized_body(Cursor::new(include_str!(concat!(
        env!("PWA_DIR"),
        "/index.js"
    ))));
    response
}

#[get("/assets/pwa/index_bg.wasm")]
pub fn pwa_wasm() -> Response<'static> {
    let mut response = Response::new();
    response.set_header(ContentType::WASM);
    let b: &'static [u8] = include_bytes!(concat!(env!("PWA_DIR"), "/index_bg.wasm"));
    response.set_sized_body(Cursor::new(b));
    response
}

#[get("/assets/pwa/manifest.json")]
pub fn pwa_manifest() -> Response<'static> {
    let mut response = Response::new();
    response.set_header(ContentType::JSON);
    response.set_sized_body(Cursor::new(include_str!(concat!(
        env!("PWA_DIR"),
        "/manifest.json"
    ))));
    response
}

#[get("/assets/pwa/sw.js")]
pub fn pwa_sw() -> Response<'static> {
    let mut response = Response::new();
    response.set_header(ContentType::JavaScript);
    response.set_sized_body(Cursor::new(include_str!(concat!(
        env!("PWA_DIR"),
        "/sw.js"
    ))));
    response
}
