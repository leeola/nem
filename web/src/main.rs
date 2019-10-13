#![feature(proc_macro_hygiene, decl_macro)]

mod foo;

//use self::foo::*;
use moxie_dom::*;
use rocket::{get, response::content::Html, routes};

use mnemosyne_gui::screens::index;
#[get("/")]
fn index() -> Html<String> {
  Html(index::render())
}

/*
#[get("/")]
fn index() -> Html<String> {
  let res = moxie_dom::render_html(move || simple_list!());
  Html(res)
}
*/

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
