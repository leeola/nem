// use crate::components::root::*;
use moxie_dom::{self, *};

#[topo::aware]
pub fn test() {
  moxie::mox! {
    <div>
      "foo"
    </div>
  }
}

pub fn render() -> String {
  moxie_dom::render_html(move || test!())
}
