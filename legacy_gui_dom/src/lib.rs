#![feature(track_caller)]

use {
    moxie,
    moxie_dom::{
        elements::{a, div, h1, header, nav},
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

#[derive(Debug)]
pub enum Route {
    A,
    B,
}

use std::ops::Deref;

#[topo::nested]
pub fn app_entry() {
    let route = state(|| Route::A);

    let closure_rc = route.clone();
    let c = Closure::wrap(Box::new(move |e: web_sys::KeyboardEvent| {
        log::info!("key down document, {:?}", e.key());
        match e.key().as_str() {
            "a" => closure_rc.update(|_| Some(Route::A)),
            "b" => closure_rc.update(|_| Some(Route::B)),
            _ => {}
        }
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
    log::info!("subbing to document keydowns");
    document().set_onkeydown(Some(c.as_ref().unchecked_ref()));
    c.forget();

    illicit::child_env![
        Key<Route> => route
    ]
    .enter(|| {
        topo::call(|| {
            moxie::mox! {
                <div>
                <header role="banner">
                    <nav role="navigation">
                        <h1><a href="/">"Nem"</a></h1>
                    </nav>
                </header>
                <router />
            </div>}
        });
    });
}

#[topo::nested]
#[illicit::from_env(route: &Key<Route>)]
pub fn router() {
    moxie::mox! {
    <div>
        {
            match route.deref() {
                Route::A => mox!{<route_a />},
                Route::B => mox!{<route_b />},
            }
        }
    </div>}
}

#[topo::nested]
pub fn route_a() {
    moxie::mox! {
    <div>
        "I'm A"
    </div>}
}

#[topo::nested]
pub fn route_b() {
    moxie::mox! {
    <div>
        "I'm B"
    </div>}
}
