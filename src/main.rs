#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use rand::{rngs::ThreadRng, Rng};
mod consts;
use consts::*;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);

    
}

fn test_event(event: Event<FormData>) {
    let data = event.data.values();
    info!("Submitted! {data:?}") 
}

#[component]
fn NewGameOptions() -> Element {
    rsx! {
        form {
            onsubmit: test_event,
            p {
                "Number of digits: ",
                select {
                    name: "digits",
                    for i in MIN_DIGITS ..= MAX_DIGITS {
                        option { {i.to_string()} }
                    }
                }
            }
            p {
                "Range of digits: ",
                select {
                    name: "range",
                    for &(k, _) in RANGES {
                        option { {k} }
                    }
                }
            }
            p {
                input {
                    r#type: "submit",
                    "Start game"
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        NewGameOptions {}
    }
}
