#![allow(dead_code)]

//! This module contains the necessary functions of DFA
//!

use std::collections::HashMap;

use crate::{
    globals::{DFAError, State},
    symbol_table::{self, Symbol, SymbolTable},
};

#[derive(Clone, Debug)]
pub struct NFA {
    num_states: usize,
    symbol_table: SymbolTable,
    // any NFA can be converted to have a single start and a single final state by
    // EPSILON transitions
    start_state: State,
    final_state: State,
    transition_function: Vec<HashMap<Symbol, State>>,
}

impl NFA {
    pub fn from_symbol(symbol: Symbol, symbol_table: SymbolTable) -> NFA {
        let mut nfa = NFA {
            num_states: 2,
            symbol_table: symbol_table.clone(),
            start_state: 0,
            final_state: 1,
            transition_function: Vec::new(),
        };

        nfa
    }
}
