use {
    serde::{Deserialize, Serialize},
    sycamore::prelude::{component, view, ReadSignal, View},
};
#[component(Foo<G>)]
pub fn my_component(value: ReadSignal<i32>) -> View<G> {
    view! {
        div(class="text-white") {
            "Value: " (value.get())
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    UniversalInput { input: String },
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    UniversalInput {
        results: Vec<universal_input::results::Result>,
    },
}
pub mod universal_input {
    use {
        results::{Result, Results},
        sycamore::prelude::{component, view, Keyed, KeyedProps, ReadSignal, Signal, View},
        wasm_bindgen::JsCast,
        web_sys::{Event, HtmlDivElement, HtmlInputElement},
    };
    #[component(UniversalInput<G>)]
    pub fn universal_input(
        (input_value, results): (Signal<String>, ReadSignal<Vec<Result>>),
    ) -> View<G> {
        let input_value_handle = input_value.handle();
        let handle_input = move |event: Event| {
            let target: HtmlDivElement = event.target().unwrap().unchecked_into();
            input_value.set(target.inner_text())
        };

        view! {
            div(class="m-1") {
                div(
                    contenteditable=true,
                    // handling the event manually because bind:value doesn't work
                    // with contentEditable. It seems to bind to `.value` but needs
                    // to bind to something like `.innerText`
                    on:input=handle_input,
                    class="m-1 bg-dark-3 text-light-2 text-2xl",
                ) {
                    (input_value_handle.get())
                }
                Results(results)
            }
        }
    }
    pub mod results {
        use {
            serde::{Deserialize, Serialize},
            sycamore::prelude::{component, view, Keyed, KeyedProps, ReadSignal, View},
        };
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Result {
            pub key: usize,
            pub message: String,
        }
        #[component(Results<G>)]
        pub fn results(results: ReadSignal<Vec<Result>>) -> View<G> {
            view! {
                ul(class="text-light-3") {
                    Keyed(KeyedProps {
                        iterable: results,
                        template: |result| view! {
                            li(class="mx-1 text-base") {
                                (result.message)
                            }
                        },
                        key: |result| result.key,
                    })
                }
            }
        }
    }
}
