//! Moore lightswitch: output depends only on state
//! Each state has fixed brightness level

use fsmall::Moore;

const LIGHT_OFF: u8 = 0;
const LIGHT_ON_DIMMED: u8 = 1;
const LIGHT_ON_MEDIUM: u8 = 2;
const LIGHT_ON_BRIGHT: u8 = 3;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Input {
    OnPress,
    OffPress,
}

static TRANSITIONS: [(u8, Input, u8); 8] = [
    (LIGHT_OFF, Input::OnPress, LIGHT_ON_DIMMED),
    (LIGHT_ON_DIMMED, Input::OffPress, LIGHT_OFF),
    (LIGHT_ON_DIMMED, Input::OnPress, LIGHT_ON_MEDIUM),
    (LIGHT_ON_MEDIUM, Input::OnPress, LIGHT_ON_BRIGHT),
    (LIGHT_ON_BRIGHT, Input::OnPress, LIGHT_ON_DIMMED),
    (LIGHT_ON_MEDIUM, Input::OffPress, LIGHT_OFF),
    (LIGHT_ON_BRIGHT, Input::OffPress, LIGHT_OFF),
    (LIGHT_OFF, Input::OffPress, LIGHT_OFF),
];

#[derive(Copy, Clone, Debug)]
enum Brightness {
    Off,
    Low,
    Medium,
    High,
}

// Moore: one output per state. Can be whatever you want.
static OUTPUTS: [Brightness; 4] = [
    Brightness::Off,    // LIGHT_OFF
    Brightness::Low,    // LIGHT_ON_DIMMED
    Brightness::Medium, // LIGHT_ON_MEDIUM
    Brightness::High,   // LIGHT_ON_BRIGHT
];

fn state_name(state: u8) -> &'static str {
    match state {
        LIGHT_OFF => "off",
        LIGHT_ON_DIMMED => "dimmed",
        LIGHT_ON_MEDIUM => "medium",
        LIGHT_ON_BRIGHT => "bright",
        _ => "unknown",
    }
}

fn main() {
    use std::io::{self, Write};

    let mut fsm = Moore::new(LIGHT_OFF, &TRANSITIONS, &OUTPUTS);

    println!("=== Moore Lightswitch ===");
    println!("Initial state: {}", state_name(fsm.current_state()));
    println!("Initial output: {:?}", fsm.current_output().unwrap());
    println!("Commands: on, off, q (quit)");

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        stdin.read_line(&mut buffer).unwrap();

        let input = match buffer.trim() {
            "on" => Input::OnPress,
            "off" => Input::OffPress,
            "q" => break,
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        match fsm.step(input) {
            Ok(brightness) => {
                println!("New state output: {:?}", brightness);
                println!("State: {}", state_name(fsm.current_state()));
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
