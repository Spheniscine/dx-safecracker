#![allow(non_snake_case)]


use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
mod consts;
use consts::*;
mod gamestate;
use gamestate::*;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);

    
}

#[component]
fn NewGameOptions(onsubmit: EventHandler<FormEvent>) -> Element {
    rsx! {
        form {
            onsubmit: move |event| onsubmit.call(event),
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
    let mut game_state: Signal<Option<GameState>> = use_signal(|| None);
    let start_game = |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let digits = values.get("digits").unwrap().as_value().parse::<usize>().unwrap_or(0).clamp(MIN_DIGITS, MAX_DIGITS);
        let range = values.get("range").unwrap().as_value();
        let range = RANGES.iter().find(|r| r.0 == range).unwrap_or(&RANGES[0]).1.clone();
        game_state.set(Some(GameState::new(digits, range)));
    };
    let guess_value = |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let value = values.get("guess").unwrap().as_value();
        if let Ok(value) = value.parse::<i32>() {
            let mut state = game_state.unwrap();
            state.guess_value(value);
            game_state.set(Some(state));
        }
    };
    let do_spin = |_event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let mut state = game_state.unwrap();
        state.spin();
        game_state.set(Some(state));
    };

    rsx! {
        if let Some(state) = game_state() {
            p {
                {format!("{:?}", state)}
            }
            p {
                "You spin the dial; it lands on this candidate code:"
            }
            h1 {
                {state.spin.to_string()}
            }
            if state.state_kind == StateKind::GuessValue {
                p {
                    "What is its value (sum of digits that match the secret code both in number and position)?"
                }
                form {
                    onsubmit: move |event| guess_value(event, &mut game_state),
                    p {
                        input {
                            r#type: "text",
                            name: "guess"
                        }
                        input {
                            r#type: "submit",
                            "Submit"
                        }
                    }
                }
            } else if state.state_kind == StateKind::GuessCode {
                if state.current_value() == 0 {
                    p {
                        "Its value is 0. You get a free guess! What is the secret code?"
                    }
                } else {
                    p {
                        "Correct! You may now guess the secret code."
                    }
                }
                form {
                    // TODO
                    p {
                        input {
                            r#type: "text",
                            name: "guess"
                        }
                        input {
                            r#type: "submit",
                            "Submit"
                        }
                    }
                }
            } else if state.state_kind == StateKind::IncorrectValue {
                p {
                    "Incorrect. The value is {state.current_value().to_string()}.", 
                }
                form {
                    onsubmit: move |event| do_spin(event, &mut game_state),
                    p {
                        button {
                            r#type: "submit",
                            "Spin"
                        }
                    }
                }
            } else if state.state_kind == StateKind::Won {
                p {
                    "You won! (TODO)"
                }
            }
        } else {
            NewGameOptions {
                onsubmit: move |event| start_game(event, &mut game_state)
            }
        }
    }
}
