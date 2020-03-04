#![feature(track_caller)]

use {
    moxie,
    moxie_dom::{
        elements::{a, div, h1, header, li, nav, ul},
        prelude::*,
    },
    wasm_bindgen::{prelude::*, JsCast},
};

#[topo::nested]
pub fn base_layout() {
    moxie::mox! {
    <div>
        <header role="banner">
            <nav role="navigation">
                <h1><a href="/">"Nem"</a></h1>
            </nav>
        </header>
    </div>}
}

#[topo::nested]
pub fn app_entry() {
    log::info!("log from gui entry");
    let c = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        log::info!("key down document, {:?}", e.key());
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    document().set_onkeydown(Some(c.as_ref().unchecked_ref()));
    c.forget();
    moxie::mox! {
        <div>
        <header role="banner">
            <nav role="navigation">
                <h1><a href="/">"Nem"</a></h1>
            </nav>
        </header>
    </div>}
}

#[topo::nested]
pub fn mox_test() {
    let items = vec!["foo", "bar"];
    moxie::mox! {<ul>{
        for item in items {
            moxie::mox!(<li>{% "{}", item }</li>)
        }
    }</ul>}
}
