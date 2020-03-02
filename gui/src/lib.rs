#![feature(track_caller)]

use {
    moxie,
    moxie_dom::{
        elements::{a, div, h1, header, input, li, nav, ul},
        prelude::*,
    },
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
pub fn electron_hover_ui() {
    log::info!("log from gui entry");
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
