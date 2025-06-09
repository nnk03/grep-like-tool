#![allow(dead_code)]

use std::collections::HashSet;
use std::ops::Index;

use crate::d_transition_function;
use crate::globals::State;
use crate::n_transition_function;
use crate::symbol_table::Symbol;

pub use d_transition_function::DTransitionFunction;
pub use n_transition_function::NTransitionFunction;

#[derive(Clone, Debug)]
pub enum TransitionFunction {
    DT(DTransitionFunction),
    NT(NTransitionFunction),
}
