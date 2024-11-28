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
    let mut alert_text = use_signal(|| String::from("Placeholder alert text"));
    let mut alert = move |msg: String| {
        alert_text.set(msg);
        eval(r#"document.getElementById("alertDialog").showModal();"#);
    };

    let mut show_history: Signal<bool> = use_signal(|| false);

    let start_game = |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let digits = values.get("digits").unwrap().as_value().parse::<usize>().unwrap_or(0).clamp(MIN_DIGITS, MAX_DIGITS);
        let range = values.get("range").unwrap().as_value();
        let range = RANGES.iter().find(|r| r.0 == range).unwrap_or(&RANGES[0]).1.clone();
        game_state.set(Some(GameState::new(digits, range)));
    };
    let mut guess_value = move |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let value = values.get("guess").unwrap().as_value();
        if let Ok(value) = value.parse::<i32>() {
            let mut state = game_state.unwrap();
            if value >= *state.range.end() as i32 * state.digits as i32 {
                alert("Guessed value is too high.".to_string());
            } else if value == 0 {
                alert("Guessed value cannot be zero.".to_string());
            } else if value < 0 {
                alert("Guessed value cannot be negative.".to_string());
            } else {
                state.guess_value(value);
                game_state.set(Some(state));
            }
        } else {
            alert("Guessed value must be a number".to_string());
        }
    };

    let mut guess_code = move |event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let values = event.data.values();
        let value = values.get("guess").unwrap().as_value();
        let mut state = game_state.unwrap();

        let parse = if value.len() != state.digits {None} else {
            let lo = b'0' + state.range.start();
            let hi = b'0' + state.range.end();
            if value.bytes().any(|b| !(lo..=hi).contains(&b)) {
                None
            } else {
                Some(Code(value.bytes().map(|x| x - b'0').collect()))
            }
        };

        if let Some(code) = parse {
            state.guess_code(code);
            game_state.set(Some(state));
        } else {
            alert(format!("Invalid code. The current game allows codes of {} digits between {} and {}, inclusive.", state.digits, state.range.start(), state.range.end()));
        }
    };

    let do_spin = |_event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        let mut state = game_state.unwrap();
        state.spin();
        game_state.set(Some(state));
    };

    let restart = |_event: Event<FormData>, game_state: &mut Signal<Option<GameState>>| {
        game_state.set(None);
    };

    rsx! {
        div { font_family: "'Trebuchet MS','Lucida Sans Unicode', 'Lucida Grande', Verdana, Arial, sans-serif",
            font_size: "12pt",
            if let Some(state) = game_state() {
                // p {
                //     //debug
                //     {format!("{:?}", state)}
                // }
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
                                r#type: "text", inputmode: "numeric",
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
                        onsubmit: move |event| guess_code(event, &mut game_state),
                        p {
                            input {
                                r#type: "text", inputmode: "numeric",
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
                } else if state.state_kind == StateKind::IncorrectCode {
                    p {
                        "Incorrect. The value of your guess is {state.last_code_value().unwrap().to_string()}.", 
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
                        "Correct! You win the game!"
                    }
                    p {
                        "You won the game in {state.spins.to_string()} spin", {if state.spins == 1 {""} else {"s"}}, "."
                    }
                    form {
                        onsubmit: move |event| restart(event, &mut game_state),
                        p {
                            button {
                                r#type: "submit",
                                "Play again"
                            }
                        }
                    }
                }

                form {
                    onsubmit: move |_event| show_history.set(!show_history()),
                    p {
                        button {
                            r#type: "submit",
                            if !show_history() {"Show History"} else {"Hide History"}
                        }
                    }
                }
                if show_history() {
                    if state.history.is_empty() { 
                        p {
                            "There are no known values yet."
                        }
                    } else {
                        ul {
                            for code in &state.history {
                                li {
                                    {code.to_string()},
                                    " → ",
                                    {state.code_value(code).to_string()}
                                }
                            }
                        }
                    }
                }
            } else {
                NewGameOptions {
                    onsubmit: move |event| start_game(event, &mut game_state)
                }
            }

            dialog {
                id: "alertDialog",
                p { {alert_text} }
                button { 
                    onclick: move |_| {
                        eval(r#"document.getElementById("alertDialog").close();"#);
                    },
                    "OK"
                }
            }
        }
    }
}
