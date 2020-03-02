#![feature(track_caller)]

use {
    moxie_dom::{self, prelude::*},
    wasm_bindgen::{prelude::*, JsCast},
};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, electron-gui!");
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Debug).expect("log failed to init");
    let c = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        log::info!("key down document, {:?}", e.key());
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    document().set_onkeydown(Some(c.as_ref().unchecked_ref()));
    moxie_dom::boot(document().body().unwrap(), mnemosyne_gui::electron_hover_ui);
    c.forget();
}
