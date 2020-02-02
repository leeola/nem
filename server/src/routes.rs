use {
    crate::handlers::{self, *},
    rocket::{routes, Route},
};

pub fn new() -> Vec<(&'static str, Vec<Route>)> {
    vec![
        ("/", routes![handlers::index, handlers::handle_mox_test]),
        #[cfg(feature = "pwa-assets")]
        (
            "/",
            routes![
                assets::pwa_html,
                assets::pwa_js,
                assets::pwa_wasm,
                assets::pwa_manifest,
                assets::pwa_sw,
            ],
        ),
    ]
}
