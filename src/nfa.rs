#![allow(dead_code)]

//! This module contains the necessary functions of DFA
//!

use std::collections::{HashMap, HashSet};

use crate::{
    globals::{DFAError, State},
    n_transition_function::NTransitionFunction,
    symbol_table::{self, Symbol, SymbolTable},
};

#[derive(Clone, Debug)]
pub struct NFA {
    num_states: usize,
    symbol_table: SymbolTable,
    // any NFA can be converted to have a single start and a single final state by
    // EPSILON transitions
    // number of states is always to be ensured as final_state - start_state + 1
    start_state: State,
    final_state: State,
    transition_function: NTransitionFunction,
}

impl NFA {
    pub fn from_symbol(symbol: &Symbol, symbol_table: &SymbolTable) -> NFA {
        let mut nfa = NFA {
            num_states: 2,
            symbol_table: symbol_table.clone(),
            start_state: 0,
            final_state: 1,
            transition_function: NTransitionFunction::new(),
        };

        nfa.transition_function.add_transition(&0, symbol, &1);

        nfa
    }

    pub fn extend(&mut self, increment: usize) {
        // increases the state number by `increment`
        for state in (self.start_state..=self.final_state).rev() {
            let removed_state = self.transition_function.f.remove_entry(&state);

            if let Some((state_key, mut transitions)) = removed_state {
                // for increment the next_state of every transition by increment
                for (_, next_states_set) in transitions.iter_mut() {
                    *next_states_set = next_states_set
                        .iter()
                        .map(|&next_state| next_state + increment)
                        .collect()
                }

                self.transition_function
                    .f
                    .insert(state_key + increment, transitions);
            }
        }

        // start and final states get incremented by `incremented`
        self.start_state += increment;
        self.final_state += increment;
    }

    /// returns an NFA corresponding to the concatenation of 2 NFAs
    pub fn concat(self, mut other: Self) -> NFA {
        let mut nfa = NFA {
            num_states: self.num_states + other.num_states,
            symbol_table: self.symbol_table,
            start_state: self.start_state,
            final_state: other.final_state,
            transition_function: self.transition_function,
        };

        // first extend the transitions correctly
        other.extend(self.num_states);

        // then add the new transitions
        nfa.transition_function.extend(other.transition_function);

        nfa
    }

    /// returns an NFA corresponding to the Kleene Star of the current NFA
    pub fn kleene_star(mut self) -> NFA {
        self.extend(1);

        self.final_state = 0;
        self.start_state = 0;

        let mut start_state_transitions: HashMap<Symbol, HashSet<State>> = HashMap::new();
        start_state_transitions.insert(Symbol::Epsilon, HashSet::from([1 as State]));

        self.transition_function
            .f
            .entry(self.start_state)
            .or_insert(start_state_transitions);

        let mut additional_final_state_transitions: HashMap<Symbol, HashSet<State>> =
            HashMap::new();
        additional_final_state_transitions.insert(Symbol::Epsilon, HashSet::from([1 as State]));

        // if final state transitions are already present extend it with the above transition
        if self.transition_function.f.contains_key(&self.final_state) {
            // if the above if is passed, then the below if will definitely get through
            if let Some(mut transitions) = self.transition_function.f.remove(&self.final_state) {
                transitions.extend(additional_final_state_transitions);
            }
        } else {
            self.transition_function
                .f
                .entry(self.final_state)
                .or_insert(additional_final_state_transitions);
        }

        self
    }

    pub fn union(mut self, mut other: Self) -> NFA {
        let mut nfa = NFA {
            num_states: self.num_states + other.num_states + 2,
            start_state: 0,
            final_state: self.num_states + other.num_states + 1,
            transition_function: NTransitionFunction::new(),
            symbol_table: self.symbol_table.clone(),
        };

        self.extend(1);
        other.extend(self.num_states + 1);

        nfa.transition_function.extend(self.transition_function);
        nfa.transition_function.extend(other.transition_function);

        // adding epsilon transitions to the start states of 2 NFA
        let mut start_state_transitions: HashMap<Symbol, HashSet<State>> = HashMap::new();
        start_state_transitions.insert(
            Symbol::Epsilon,
            HashSet::from([self.start_state, other.start_state]),
        );

        nfa.transition_function
            .f
            .entry(0 as State)
            .or_insert(start_state_transitions);

        let mut additional_final_state_transitions: HashMap<Symbol, HashSet<State>> =
            HashMap::new();
        additional_final_state_transitions
            .insert(Symbol::Epsilon, HashSet::from([nfa.final_state]));

        // adding Epsilon transitions from final state of the 2 NFAs to the final state of new NFA

        // if final state transitions are already present extend it with the above transition
        if nfa.transition_function.f.contains_key(&self.final_state) {
            // if the above if is passed, then the below if will definitely get through
            if let Some(mut transitions) = nfa.transition_function.f.remove(&self.final_state) {
                transitions.extend(additional_final_state_transitions.clone());
            }
        } else {
            nfa.transition_function
                .f
                .entry(self.final_state)
                .or_insert(additional_final_state_transitions.clone());
        }

        // if final state transitions are already present extend it with the above transition
        if nfa.transition_function.f.contains_key(&other.final_state) {
            // if the above if is passed, then the below if will definitely get through
            if let Some(mut transitions) = nfa.transition_function.f.remove(&other.final_state) {
                transitions.extend(additional_final_state_transitions);
            }
        } else {
            nfa.transition_function
                .f
                .entry(other.final_state)
                .or_insert(additional_final_state_transitions);
        }

        nfa
    }
}
