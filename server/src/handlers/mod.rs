pub mod acme_challenge;

use {
  crate::{error::Result, states::Template},
  rocket::{get, response::content::Html, State},
};

#[get("/")]
pub fn index(tmpl: State<Template>) -> Result<Html<String>> {
  let tmpl_s = tmpl.render("index", ())?;
  Ok(Html(tmpl_s))
}
