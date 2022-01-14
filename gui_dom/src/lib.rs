use sycamore::prelude::{component, view, ReadSignal, View};
#[component(Foo<G>)]
pub fn my_component(value: ReadSignal<i32>) -> View<G> {
    view! {
        div(class="text-white") {
            "Value: " (value.get())
        }
    }
}
pub mod universal_input {
    pub mod results {
        use sycamore::prelude::{component, view, Keyed, KeyedProps, ReadSignal, View};
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct Result {
            pub key: usize,
            pub message: String,
        }
        #[component(Results<G>)]
        pub fn results(results: ReadSignal<Vec<Result>>) -> View<G> {
            view! {
                ul(class="text-white") {
                    Keyed(KeyedProps {
                        iterable: results,
                        template: |result| view! {
                            li { (result.message) }
                        },
                        key: |result| result.key,
                    })
                }
            }
        }
    }
}
