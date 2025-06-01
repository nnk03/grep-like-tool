#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::{globals::State, symbol_table::Symbol};

// Non Deterministic Transition Function
#[derive(Clone, Debug)]
pub struct NTransitionFunction {
    pub f: HashMap<State, HashMap<Symbol, HashSet<State>>>,
}

impl NTransitionFunction {
    /// returns a new instance of Non Deterministic Transition function
    pub fn new() -> NTransitionFunction {
        NTransitionFunction { f: HashMap::new() }
    }

    /// Adds a transition curr_state to next_state on seeing the alphabet `symbol`
    pub fn add_transition(&mut self, curr_state: &State, symbol: &Symbol, next_state: &State) {
        let symbol_state_transition = self.f.entry(*curr_state).or_insert(HashMap::new());
        let entry = symbol_state_transition
            .entry(*symbol)
            .or_insert(HashSet::new());

        entry.insert(*next_state);
    }

    /// returns None if invalid transition else Some(set of allowed transitions)
    pub fn get(&self, curr_state: &State, symbol: &Symbol) -> Option<&HashSet<State>> {
        if !self.f.contains_key(curr_state) {
            return None;
        }

        if !self.f[curr_state].contains_key(symbol) {
            return None;
        }

        Some(&self.f[curr_state][symbol])
    }

    /// returns the length of the vector
    fn len(&self) -> usize {
        self.f.len()
    }

    /// extends a transition function
    pub fn extend(&mut self, other_transition_function: NTransitionFunction) {
        self.f.extend(other_transition_function.f);
    }
}
