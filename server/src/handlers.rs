pub mod acme_challenge;
#[cfg(feature = "pwa-assets")]
pub mod assets;

use {
  crate::error::Result,
  mnemosyne_gui::*,
  moxie_dom::*,
  rocket::{get, response::content::Html, State},
};

#[get("/")]
pub fn index(tmpl: State<handlebars::Handlebars>) -> Result<Html<String>> {
  let tmpl_s = tmpl.render("index", &()).unwrap();
  Ok(Html(tmpl_s))
}

#[get("/mox_test")]
pub fn handle_mox_test() -> Html<String> {
  let res = moxie_dom::render_html(move || mnemosyne_gui::mox_test!());
  Html(res)
}
