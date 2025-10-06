//! Mealy lightswitch: output depends on (state, input)
//! Pressing ON cycles brightness, pressing OFF turns light off

use fsmall::Mealy;

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

static OUTPUTS: [(u8, Input, Brightness); 8] = [
    (LIGHT_OFF, Input::OnPress, Brightness::Low),
    (LIGHT_ON_DIMMED, Input::OffPress, Brightness::Off),
    (LIGHT_ON_DIMMED, Input::OnPress, Brightness::Medium),
    (LIGHT_ON_MEDIUM, Input::OnPress, Brightness::High),
    (LIGHT_ON_BRIGHT, Input::OnPress, Brightness::Low),
    (LIGHT_ON_MEDIUM, Input::OffPress, Brightness::Off),
    (LIGHT_ON_BRIGHT, Input::OffPress, Brightness::Off),
    (LIGHT_OFF, Input::OffPress, Brightness::Off),
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

    let mut fsm = Mealy::new(LIGHT_OFF, &TRANSITIONS, &OUTPUTS);

    println!("=== Mealy Lightswitch ===");
    println!("Initial state: {}", state_name(fsm.current_state()));
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
                println!("Output: {:?}", brightness);
                println!("State: {}", state_name(fsm.current_state()));
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
