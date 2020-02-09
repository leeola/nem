#![feature(track_caller)]

mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, electron-gui!");
}

use {
    moxie_dom::{
        self,
        elements::{li, ul},
        prelude::*,
    },
    wasm_bindgen::prelude::*,
};

#[topo::nested]
fn simple_list(items: &[String]) {
    moxie::mox! {
        <ul>{
            for item in items {
                moxie::mox!(<li>{% "{}", item }</li>)
            }
        }</ul>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    moxie_dom::boot(document().body().unwrap(), || {
        simple_list(&["hello from wasm and moxie".to_owned()])
    });
}
