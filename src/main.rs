#![allow(non_snake_case)]

use std::ops::{Deref, RangeInclusive};

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

type Code = Vec<u8>;

#[derive(Debug, Clone)]
struct GameState {
    digits: usize,
    range: RangeInclusive<u8>,
    secret: Code,
    spin: Code
}

fn random_code(digits: usize, range: RangeInclusive<u8>) -> Code {
    let mut rng = ThreadRng::default();
    (0..digits).map(|_| rng.gen_range(range.clone())).collect()
}

impl GameState {
    fn new(digits: usize, range: RangeInclusive<u8>) -> Self {
        Self {
            digits,
            range: range.clone(),
            secret: random_code(digits, range.clone()),
            spin: random_code(digits, range.clone()),
        }
    }
}

#[component]
fn NewGameOptions() -> Element {
    let mut game_state = use_signal(|| None);
    let start_game = |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let digits = values.get("digits").unwrap().as_value().parse::<usize>().unwrap_or(0).clamp(MIN_DIGITS, MAX_DIGITS);
        let range = values.get("range").unwrap().as_value();
        let range = RANGES.iter().find(|r| r.0 == range).unwrap_or(&RANGES[0]).1.clone();
        game_state.set(Some(GameState::new(digits, range)));
    };


    rsx! {
        if let Some(state) = game_state() {
            p {
                {format!("{:?}", state)}
            }
        } else {
            form {
                onsubmit: move |event| start_game(event, &mut game_state),
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
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        NewGameOptions {}
    }
}
