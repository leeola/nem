#![feature(track_caller)]

use {
    moxie_dom::{self, prelude::*},
    wasm_bindgen::prelude::*,
};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, electron-gui!");
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Debug).expect("log failed to init");
    moxie_dom::boot(document().body().unwrap(), mnemosyne_gui::app_entry);
}
