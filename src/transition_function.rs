#![allow(dead_code)]

use crate::custom_errors::AutomatonError;
use crate::d_transition_function;
use crate::n_transition_function;
use crate::state::State;
use crate::symbol_table::Symbol;

pub use d_transition_function::DTransitionFunction;
pub use n_transition_function::NTransitionFunction;

#[derive(Clone, Debug)]
pub enum TransitionFunction {
    DT(DTransitionFunction),
    NT(NTransitionFunction),
}

pub trait BasicFunctionsForTransitions {
    fn new() -> Self;

    fn add_transition(
        &mut self,
        state: &State,
        symbol: &Symbol,
        next_state: &State,
    ) -> Result<(), AutomatonError>;

    fn extend(&mut self, increment: usize);
}
