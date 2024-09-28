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

#[component]
fn NewGameOptions() -> Element {
    rsx! {
        form {
            onsubmit: move |event| { 
                let data = event.data.values();
                info!("Submitted! {data:?}") 
            },
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
                    option { "1-6" }
                    option { "1-4" }
                    option { "1-9" }
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
