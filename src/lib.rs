//! # fsmall
//!
//! Small finite state machine library with no allocations.
//! Supports both Mealy and Moore machines.
//!
//! ## Features
//! - Zero heap allocations
//! - No standard library required (no_std compatible)
//! - Static transition and output tables
//! - Explicit error handling
//!
//! ## Example (Mealy)
//! ```
//! use fsmall::Mealy;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! enum Input { A, B }
//!
//! #[derive(Copy, Clone, Debug, PartialEq)]  // Added PartialEq here
//! enum Output { X, Y }
//!
//! static TRANSITIONS: [(u8, Input, u8); 2] = [
//!     (0, Input::A, 1),
//!     (1, Input::B, 0),
//! ];
//!
//! static OUTPUTS: [(u8, Input, Output); 2] = [
//!     (0, Input::A, Output::X),
//!     (1, Input::B, Output::Y),
//! ];
//!
//! let mut fsm = Mealy::new(0, &TRANSITIONS, &OUTPUTS);
//! assert_eq!(fsm.step(Input::A), Ok(Output::X));
//! ```

#![no_std]

/// Error returned when FSM step fails
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StepError {
    /// No transition defined for (state, input) pair
    NoTransition,
    /// No output defined for (state, input) pair in Mealy
    /// or state index out of bounds in Moore
    NoOutput,
}

/// Mealy machine: output depends on (current_state, input)
pub struct Mealy<I: 'static, O: 'static> {
    state: u8,
    // Table: (from_state, input, to_state)
    transitions: &'static [(u8, I, u8)],
    // Table: (state, input, output)
    outputs: &'static [(u8, I, O)],
}

impl<I: Copy + Eq + 'static, O: Copy + 'static> Mealy<I, O> {
    /// Create new Mealy machine
    ///
    /// # Arguments
    /// * `initial_state` - Starting state (0-255)
    /// * `transitions` - Transition table: (from_state, input, to_state)
    /// * `outputs` - Output table: (state, input, output)
    pub fn new(
        initial_state: u8,
        transitions: &'static [(u8, I, u8)],
        outputs: &'static [(u8, I, O)],
    ) -> Self {
        Mealy {
            state: initial_state,
            transitions,
            outputs,
        }
    }

    /// Process input, transition to next state, return output
    ///
    /// # Errors
    /// * `StepError::NoTransition` - No rule for (state, input)
    /// * `StepError::NoOutput` - No output for (state, input)
    pub fn step(&mut self, input: I) -> Result<O, StepError> {
        // Find next state in transition table
        let next = self
            .transitions
            .iter()
            .find(|(from, inp, _to)| *from == self.state && *inp == input)
            .map(|(_from, _inp, to)| *to)
            .ok_or(StepError::NoTransition)?;

        // Find output in output table
        let output = self
            .outputs
            .iter()
            .find(|(s, i, _o)| *s == self.state && *i == input)
            .map(|(_s, _i, o)| *o)
            .ok_or(StepError::NoOutput)?;

        // Commit state transition
        self.state = next;

        Ok(output)
    }

    /// Get current state
    pub fn current_state(&self) -> u8 {
        self.state
    }

    /// Reset to specific state
    pub fn reset(&mut self, state: u8) {
        self.state = state;
    }
}

/// Moore machine: output depends only on current_state
pub struct Moore<I: 'static, O: 'static> {
    state: u8,
    // Table: (from_state, input, to_state)
    transitions: &'static [(u8, I, u8)],
    // Array: outputs[state] = output
    outputs: &'static [O],
}

impl<I: Copy + Eq + 'static, O: Copy + 'static> Moore<I, O> {
    /// Create new Moore machine
    ///
    /// # Arguments
    /// * `initial_state` - Starting state (0-255)
    /// * `transitions` - Transition table: (from_state, input, to_state)
    /// * `outputs` - Output array: index=state, value=output
    pub fn new(
        initial_state: u8,
        transitions: &'static [(u8, I, u8)],
        outputs: &'static [O],
    ) -> Self {
        Moore {
            state: initial_state,
            transitions,
            outputs,
        }
    }

    /// Process input, transition to next state, return new state's output
    ///
    /// # Errors
    /// * `StepError::NoTransition` - No rule for (state, input)
    /// * `StepError::NoOutput` - Next state index out of bounds
    pub fn step(&mut self, input: I) -> Result<O, StepError> {
        // Find next state in transition table
        let next = self
            .transitions
            .iter()
            .find(|(from, inp, _to)| *from == self.state && *inp == input)
            .map(|(_from, _inp, to)| *to)
            .ok_or(StepError::NoTransition)?;

        // Commit state transition
        self.state = next;

        // Get output for new state
        self.outputs
            .get(self.state as usize)
            .copied()
            .ok_or(StepError::NoOutput)
    }

    /// Get current state
    pub fn current_state(&self) -> u8 {
        self.state
    }

    /// Get current output (without transitioning)
    pub fn current_output(&self) -> Result<O, StepError> {
        self.outputs
            .get(self.state as usize)
            .copied()
            .ok_or(StepError::NoOutput)
    }

    /// Reset to specific state
    pub fn reset(&mut self, state: u8) {
        self.state = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, Eq, PartialEq)]
    enum TestInput {
        A,
        B,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum TestOutput {
        X,
        Y,
    }

    // Mealy tests
    static MEALY_TRANS: [(u8, TestInput, u8); 2] = [(0, TestInput::A, 1), (1, TestInput::B, 0)];

    static MEALY_OUTS: [(u8, TestInput, TestOutput); 2] = [
        (0, TestInput::A, TestOutput::X),
        (1, TestInput::B, TestOutput::Y),
    ];

    #[test]
    fn mealy_valid_transition() {
        let mut fsm = Mealy::new(0, &MEALY_TRANS, &MEALY_OUTS);
        assert_eq!(fsm.step(TestInput::A), Ok(TestOutput::X));
        assert_eq!(fsm.current_state(), 1);
    }

    #[test]
    fn mealy_invalid_transition() {
        let mut fsm = Mealy::new(0, &MEALY_TRANS, &MEALY_OUTS);
        assert_eq!(fsm.step(TestInput::B), Err(StepError::NoTransition));
        assert_eq!(fsm.current_state(), 0); // State unchanged on error
    }

    #[test]
    fn mealy_reset() {
        let mut fsm = Mealy::new(0, &MEALY_TRANS, &MEALY_OUTS);
        fsm.step(TestInput::A).unwrap();
        assert_eq!(fsm.current_state(), 1);
        fsm.reset(0);
        assert_eq!(fsm.current_state(), 0);
    }

    // Moore tests
    static MOORE_TRANS: [(u8, TestInput, u8); 2] = [(0, TestInput::A, 1), (1, TestInput::B, 0)];

    static MOORE_OUTS: [TestOutput; 2] = [TestOutput::X, TestOutput::Y];

    #[test]
    fn moore_valid_transition() {
        let mut fsm = Moore::new(0, &MOORE_TRANS, &MOORE_OUTS);
        // Transition to state 1, get its output (Y)
        assert_eq!(fsm.step(TestInput::A), Ok(TestOutput::Y));
        assert_eq!(fsm.current_state(), 1);
    }

    #[test]
    fn moore_current_output() {
        let fsm = Moore::new(0, &MOORE_TRANS, &MOORE_OUTS);
        assert_eq!(fsm.current_output(), Ok(TestOutput::X));
    }

    #[test]
    fn moore_invalid_transition() {
        let mut fsm = Moore::new(0, &MOORE_TRANS, &MOORE_OUTS);
        assert_eq!(fsm.step(TestInput::B), Err(StepError::NoTransition));
    }
}
