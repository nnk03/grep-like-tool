#![allow(dead_code)]

use std::collections::HashSet;

use crate::{globals::State, symbol_table::SymbolTable, transition_function::NTransitionFunction};

#[derive(Clone, Debug)]
pub struct NFA {
    num_states: usize,
    symbol_table: SymbolTable,
    // set of states
    states: HashSet<State>,
    // state numbers start from this
    begin_state_num: State,
    // upto end_state_num
    end_state_num: State,
    // do we want multiple start states or single start state
    // do we want multiple end states or a single end state
    transition_function: NTransitionFunction,
}
