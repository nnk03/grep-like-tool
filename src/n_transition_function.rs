#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::{globals::State, symbol_table::Symbol};

#[derive(Debug)]
pub struct NTransitionFunction {
    f: Vec<HashMap<Symbol, HashSet<State>>>,
}

impl NTransitionFunction {
    /// returns a new instance of Non Deterministic Transition function
    pub fn new() -> NTransitionFunction {
        NTransitionFunction { f: Vec::new() }
    }

    /// Adds a transition curr_state to next_state on seeing the alphabet `symbol`
    pub fn add_transition(&mut self, curr_state: State, symbol: Symbol, next_state: State) {
        if curr_state >= self.len() {
            self.f.resize(curr_state + 1, HashMap::new());
        }

        let entry = self.f[curr_state].entry(symbol).or_insert(HashSet::new());
        entry.insert(next_state);
    }

    /// returns None if invalid transition else Some(set of allowed transitions)
    pub fn get(&self, curr_state: State, symbol: Symbol) -> Option<&HashSet<State>> {
        if curr_state >= self.len() {
            return None;
        }

        if !self.f[curr_state].contains_key(&symbol) {
            return None;
        }

        Some(&self.f[curr_state][&symbol])
    }

    /// returns the length of the vector
    fn len(&self) -> usize {
        self.f.len()
    }
}
