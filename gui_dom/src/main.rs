use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            p(class="text-3xl font-bold underline") { "foo! zoz perf reso" }
        }
    });
}
