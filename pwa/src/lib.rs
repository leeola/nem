use {moxie_dom::*, wasm_bindgen::prelude::*};

#[topo::nested]
fn todo_app() {
  moxie::mox! {
    <div>
      "content loaded from moxie"
    </div>
  }
}

#[wasm_bindgen(start)]
pub fn main() {
  moxie_dom::boot(document().body().unwrap(), || todo_app!());
}
