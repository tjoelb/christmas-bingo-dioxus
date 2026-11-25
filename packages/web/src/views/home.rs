use dioxus::prelude::*;

const BINGO_LETTERS: [char; 5] = ['B', 'I', 'N', 'G', 'O'];

#[component]
pub fn Home() -> Element {
    let called = use_signal(Vec::<u8>::new);
    let order = use_signal(shuffled_numbers);
    let mut running = use_signal(|| false);
    let mut speed = use_signal(|| 5_u32);
    let mut run_id = use_signal(|| 0_u64);
    let current_call = use_signal(|| None::<u8>);
    let previous_calls = use_signal(Vec::<String>::new);
    let call_index = use_signal(|| 0_u32);
    let ball_color = use_signal(|| "#a51d2d".to_string());
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (order, call_index);

    let status = if called().len() == 75 {
        "All numbers called"
    } else if running() {
        "Calling"
    } else {
        "Ready to call"
    };

    rsx! {
        div { class: "bingo-app",
            aside { class: "control-panel",
                div { class: "brand",
                    div { class: "brand-star", "★" }
                    div { class: "brand-copy",
                        span { "HOLIDAY" }
                        h1 { "Bingo" }
                    }
                }

                div { class: "controls",
                    div { class: "primary-controls",
                        button {
                            class: "start-button",
                            onclick: move |_| {
                                if called().len() < 75 && !running() {
                                    let current_run = run_id().wrapping_add(1);
                                    run_id.set(current_run);
                                    running.set(true);
                                    #[cfg(target_arch = "wasm32")]
                                    {
                                        let mut called = called;
                                        let order = order;
                                        let mut running = running;
                                        let speed = speed;
                                        let run_id = run_id;
                                        let mut current_call = current_call;
                                        let previous_calls = previous_calls;
                                        let mut call_index = call_index;
                                        let mut ball_color = ball_color;
                                        spawn(async move {
                                            loop {
                                                if !running() || run_id() != current_run {
                                                    break;
                                                }
                                                let delay = speed().max(1).saturating_mul(1000);
                                                gloo_timers::future::TimeoutFuture::new(delay).await;
                                                if !running() || run_id() != current_run {
                                                    break;
                                                }

                                                let next = order()
                                                    .into_iter()
                                                    .find(|number| !called().contains(number));
                                                let Some(number) = next else {
                                                    running.set(false);
                                                    break;
                                                };

                                                if let Some(previous) = current_call() {
                                                    record_previous(
                                                        previous_calls,
                                                        format!("{} {}", letter_for(previous), previous),
                                                    );
                                                }

                                                called.with_mut(|numbers| numbers.push(number));
                                                current_call.set(Some(number));
                                                let index = call_index();
                                                call_index.set(index.saturating_add(1));
                                                ball_color.set(if index % 2 == 0 {
                                                    "#a51d2d".to_string()
                                                } else {
                                                    "green".to_string()
                                                });
                                                speak_number(number);

                                                if called().len() == 75 {
                                                    running.set(false);
                                                    break;
                                                }
                                            }
                                        });
                                    }
                                }
                            },
                            "START"
                        }
                        button {
                            class: "stop-button",
                            onclick: move |_| {
                                running.set(false);
                                run_id.with_mut(|value| *value = value.wrapping_add(1));
                            },
                            "STOP"
                        }
                    }

                    fieldset { class: "speed-control",
                        legend { "Call speed (seconds)" }
                        input {
                            r#type: "number",
                            min: "1",
                            value: "{speed}",
                            aria_label: "Call speed (seconds)",
                            oninput: move |event| {
                                if let Ok(value) = event.value().parse::<u32>() {
                                    speed.set(value.max(1));
                                }
                            },
                        }
                    }

                    div { class: "quick-controls", "aria-label": "Quick call controls",
                        span { "QUICK CALL" }
                        div { class: "quick-buttons",
                            for letter in ['I', 'N', 'G'] {
                                button {
                                    class: "quick-button",
                                    aria_label: "Mark all {letter} numbers called",
                                    onclick: move |_| {
                                        let start = match letter {
                                            'I' => 16,
                                            'N' => 31,
                                            _ => 46,
                                        };
                                        mark_numbers(called, start..start + 15);
                                    },
                                    "{letter}"
                                }
                            }
                        }
                    }
                }

                div { class: "called-count",
                    strong { "{called().len()}" }
                    span { "NUMBERS CALLED" }
                }
            }

            section { class: "game-area",
                header { class: "page-heading",
                    div {
                        p { "THE NORTH POLE BINGO HALL" }
                        h2 { "Christmas Call Board" }
                    }
                    div {
                        class: if running() { "status status-active" } else { "status" },
                        aria_live: "polite",
                        span { class: "status-dot" }
                        "{status}"
                    }
                }

                main { class: "game-layout",
                    section { class: "bingo-board", aria_label: "Bingo number board",
                        for (column, letter) in BINGO_LETTERS.iter().enumerate() {
                            table { class: "number-column",
                                thead {
                                    tr { th { class: "column-{letter}", scope: "col", "{letter}" } }
                                }
                                tbody {
                                    for number in (column * 15 + 1)..=(column + 1) * 15 {
                                        {
                                            let number = number as u8;
                                            let letter = letter_for(number);
                                            let is_called = called().contains(&number);
                                            rsx! {
                                                tr {
                                                    td {
                                                        button {
                                                            class: if is_called { "number-cell called" } else { "number-cell" },
                                                            aria_label: "Speak {letter} {number}",
                                                            onclick: move |_| speak_number(number),
                                                            "{number}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    section { class: "call-details",
                        article { class: "current-call",
                            span { class: "section-label", "CURRENT NUMBER" }
                            if let Some(number) = current_call() {
                                div {
                                    class: "current-ball",
                                    style: "--ball-color: {ball_color}",
                                    span { "{letter_for(number)}" }
                                    strong { "{number}" }
                                }
                            } else {
                                div { class: "current-ball empty-ball",
                                    span { "—" }
                                    strong { "—" }
                                }
                            }
                        }
                        article { class: "previous-calls",
                            span { class: "section-label", "PREVIOUS" }
                            if previous_calls().is_empty() {
                                p { class: "waiting-message", "Waiting..." }
                            } else {
                                ul {
                                    for call in previous_calls() {
                                        li { "{call}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn shuffled_numbers() -> Vec<u8> {
    let mut numbers = (1..=75).collect::<Vec<u8>>();
    #[cfg(target_arch = "wasm32")]
    let mut seed = (js_sys::Math::random() * u64::MAX as f64) as u64;
    #[cfg(not(target_arch = "wasm32"))]
    let mut seed = 0x9e37_79b9_7f4a_7c15_u64;

    for index in (1..numbers.len()).rev() {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        numbers.swap(index, (seed % (index as u64 + 1)) as usize);
    }

    numbers
}

fn letter_for(number: u8) -> char {
    BINGO_LETTERS[(number as usize - 1) / 15]
}

fn mark_numbers(mut called: Signal<Vec<u8>>, numbers: impl IntoIterator<Item = u8>) {
    called.with_mut(|called_numbers| {
        for number in numbers {
            if !called_numbers.contains(&number) {
                called_numbers.push(number);
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn record_previous(mut previous_calls: Signal<Vec<String>>, call: String) {
    previous_calls.with_mut(|calls| {
        calls.insert(0, call);
        calls.truncate(3);
    });
}

#[cfg(target_arch = "wasm32")]
fn speak_number(number: u8) {
    let phrase = format!("{} {}", letter_for(number), number).replace("67", "6 7");
    speak_text(&phrase);
}

#[cfg(not(target_arch = "wasm32"))]
fn speak_number(_: u8) {}

#[cfg(target_arch = "wasm32")]
fn speak_text(text: &str) {
    let Some(window) = web_sys::window() else {
        web_sys::console::error_1(&"Speech synthesis is unavailable: no browser window.".into());
        return;
    };
    let synthesis = match window.speech_synthesis() {
        Ok(synthesis) => synthesis,
        Err(error) => {
            web_sys::console::error_1(
                &format!("Speech synthesis is unavailable: {error:?}").into(),
            );
            return;
        }
    };
    let utterance = match web_sys::SpeechSynthesisUtterance::new_with_text(text) {
        Ok(utterance) => utterance,
        Err(error) => {
            web_sys::console::error_1(
                &format!("Could not create speech utterance: {error:?}").into(),
            );
            return;
        }
    };

    utterance.set_lang("en-US");
    utterance.set_rate(1.0);
    synthesis.speak(&utterance);
}
