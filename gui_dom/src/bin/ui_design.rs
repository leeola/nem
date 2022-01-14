use sycamore::prelude::*;

fn main() {
    use nem_gui_dom::universal_input::results::{Result, Results};
    let state = Signal::new(vec![
        Result {
            key: 0,
            message: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.".into(),
        },
        Result {
            key: 1,
            message: "Ut quis facilisis urna, sed finibus urna. Morbi nec rutrum nisl, ac consequat nunc.".into(),
        },
    ]);
    sycamore::render(|| {
        view! {
            Results(state.handle())
        }
    });
}
