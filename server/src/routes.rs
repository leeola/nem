use {
  crate::handlers::{self, *},
  rocket::{routes, Route},
};

pub fn new() -> Vec<(&'static str, Vec<Route>)> {
  vec![
    ("/", routes![handlers::index, handlers::handle_mox_test]),
    #[cfg(feature = "pwa-assets")]
    ("/", routes![assets::pwa_index]),
  ]
}
