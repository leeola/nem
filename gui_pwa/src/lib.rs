use {mnemosyne_gui::*, moxie_dom::*, wasm_bindgen::prelude::*};

#[wasm_bindgen(start)]
pub fn main() {
    moxie_dom::boot(document().body().unwrap(), || base_layout!());
}
