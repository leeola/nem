pub mod acme_challenge;

use {
  crate::error::Result,
  moxie,
  moxie_dom::*,
  rocket::{get, response::content::Html, State},
};

#[get("/")]
pub fn index(tmpl: State<handlebars::Handlebars>) -> Result<Html<String>> {
  let tmpl_s = tmpl.render("index", &()).unwrap();
  Ok(Html(tmpl_s))
}

#[topo::nested]
fn mox_test() {
  let items = vec!["foo", "bar"];
  moxie::mox! {
    <ul>{
      for item in items {
        moxie::mox!(<li>{% "{}", item }</li>)
      }
    }</ul>
  }
}

#[get("/mox_test")]
pub fn handle_mox_test() -> Html<String> {
  let res = moxie_dom::render_html(move || mox_test!());
  Html(res)
}
