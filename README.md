# fsmall

Small finite state machine library with no allocations. Supports Mealy and Moore machines.

## Features

- Zero heap allocations
- `no_std` compatible
- Static transition and output tables
- Explicit error handling
- 256 states maximum (u8)

## Examples

```sh
cargo run --example lightswitch_mealy
cargo run --example lightswitch_moore
```

## Usage

### Moore Machine

Output depends on current state.

Used when output is fixed per state, for example to display values.


```rust
use fsmall::Moore;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Input { A, B }

#[derive(Copy, Clone, Debug, PartialEq)]
enum Output { X, Y }

// Transition table: (from_state, input, to_state)
static TRANSITIONS: [(u8, Input, u8); 2] = [
    (0, Input::A, 1),
    (1, Input::B, 0),
];

// Output array: outputs[state] = output
static OUTPUTS: [Output; 2] = [
    Output::X,  // State 0 outputs X
    Output::Y,  // State 1 outputs Y
];

let mut fsm = Moore::new(0, &TRANSITIONS, &OUTPUTS);

// Get current output without transitioning
assert_eq!(fsm.current_output(), Ok(Output::X));

// Transition to state 1, get new state's output
assert_eq!(fsm.step(Input::A), Ok(Output::Y));
assert_eq!(fsm.current_state(), 1);
```


### Mealy Machine

Output depends on both current state and input.

Used when output changes based on transition, for example in events and protocol handlers.


```rust
use fsmall::Mealy;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Input { A, B }

#[derive(Copy, Clone, Debug, PartialEq)]
enum Output { X, Y }

// Transition table: (from_state, input, to_state)
static TRANSITIONS: [(u8, Input, u8); 2] = [
    (0, Input::A, 1),
    (1, Input::B, 0),
];

// Output table: (state, input, output)
static OUTPUTS: [(u8, Input, Output); 2] = [
    (0, Input::A, Output::X),
    (1, Input::B, Output::Y),
];

let mut fsm = Mealy::new(0, &TRANSITIONS, &OUTPUTS);
assert_eq!(fsm.step(Input::A), Ok(Output::X));
assert_eq!(fsm.current_state(), 1);
```


