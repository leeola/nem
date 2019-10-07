#![feature(proc_macro_hygiene, decl_macro)]

use moxie_dom::*;
use rocket::{get, response::content::Html, routes};

#[topo::aware]
fn simple_list() {
  let items = vec!["foo", "bar"];
  moxie::mox! {
    <ul>{
      for item in items {
        moxie::mox!(<li>{% "{}", item }</li>)
      }
    }</ul>
  }
}

#[get("/")]
fn index() -> Html<String> {
  let res = moxie_dom::render_html(move || simple_list!());
  Html(res)
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
