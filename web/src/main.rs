#![feature(proc_macro_hygiene, decl_macro)]

pub mod error;
pub mod state;

use {
  self::{
    error::{Error, Result},
    state::template::Template,
  },
  rocket::{get, response::content::Html, routes, State},
  rocket_contrib::serve::StaticFiles,
};

#[get("/")]
fn index(tmpl: State<Template>) -> std::result::Result<Html<String>, Error> {
  let tmpl_s = tmpl.render("index", ())?;
  Ok(Html(tmpl_s))
}

fn main() {
  rocket::ignite()
    .mount("/public", StaticFiles::from("./public"))
    .manage(Template::new("./templates").expect("templates failed to initialize"))
    .mount("/", routes![index])
    .launch();
}
