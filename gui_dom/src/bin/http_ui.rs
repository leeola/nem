use sycamore::prelude::*;

fn main() {
    sycamore::render(|| {
        view! {
            p(class="dark:bg-black dark:text-red text-3xl font-bold underline") { "http ui foo! zoz perf reso woo" }
        }
    });
}
