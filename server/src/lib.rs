use axum::{routing::get, Router};
use std::net::SocketAddr;

pub fn app() -> Router {
    Router::new().route("/", get(root))
}
pub async fn run() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app().into_make_service())
        .await
        .unwrap();
}

pub async fn root() -> &'static str {
    "Hello, World!"
}
