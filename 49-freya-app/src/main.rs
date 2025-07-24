#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn app() -> Element {
    let mut state = use_signal(|| 0);
    let onclick = move |_| {
        state += 1;
        *state.write() += 1;
    };

    println!("{}", state);
    println!("{}", state.read());

    rsx!(
        label {
            onclick,
            "State is {state}"
        }
    )    
}

fn main() {
    launch(app);
}
