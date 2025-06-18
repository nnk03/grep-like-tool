#![allow(dead_code)]

use crate::{
    custom_errors::{AutomatonError, NFAError},
    state::State,
    symbol_table::Symbol,
    transition_function::BasicFunctionsForTransitions,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct NTransitionFunction {
    pub f: HashMap<State, HashMap<Symbol, HashSet<State>>>,
}

impl BasicFunctionsForTransitions for NTransitionFunction {
    fn new() -> Self {
        NTransitionFunction { f: HashMap::new() }
    }

    fn extend(&mut self, increment: usize) {
        //
        let mut on_states: Vec<State> = self.f.keys().map(|&val| val).collect();
        on_states.sort();
        // going through keys in decreasing order, in order to avoid overlapping issue
        on_states.reverse();

        for &curr_state in on_states.iter() {
            if let Some((state, symbol_to_next_state_set_map)) = self.f.remove_entry(&curr_state) {
                let mut new_transitions: HashMap<Symbol, HashSet<State>> = HashMap::new();

                // (curr_state, symbol) -> next_state_set
                for (symbol, next_state_set) in symbol_to_next_state_set_map.iter() {
                    // new_transitions.insert(*symbol, *next_state + increment);
                    let mut new_next_state_set = HashSet::new();

                    for &next_state in next_state_set.iter() {
                        new_next_state_set.insert(next_state + increment);
                    }

                    new_transitions.insert(*symbol, new_next_state_set);
                }

                // update the transitions
                self.f.insert(state + increment, new_transitions);
            }
        }
    }

    fn add_transition(
        &mut self,
        state: &State,
        symbol: &Symbol,
        next_state: &State,
    ) -> Result<(), AutomatonError> {
        let state_transitions = self.f.entry(*state).or_insert(HashMap::new());
        let state_symbol_transitions = state_transitions.entry(*symbol).or_insert(HashSet::new());

        if state_symbol_transitions.contains(next_state) {
            return Err(AutomatonError::NFAError(NFAError::ExistingTransition(
                "".to_string(),
            )));
        }

        state_symbol_transitions.insert(*next_state);
        Ok(())
    }
}

impl NTransitionFunction {
    /// takes in self and another NTransitionFunction and returns the combined transition table of the 2
    pub fn combine_transition(mut self, other: &Self) -> Self {
        for (&state, other_transitions) in other.f.iter() {
            let existing_transitions = self.f.entry(state).or_insert(HashMap::new());

            for (&symbol, next_states) in other_transitions.iter() {
                let entry = existing_transitions.entry(symbol).or_insert(HashSet::new());
                for &next_state in next_states.iter() {
                    entry.insert(next_state);
                }
            }
        }

        self
    }

    /// to check if a transition is valid, on a state and symbol
    pub fn is_valid_transition(&self, state: &State, symbol: &Symbol) -> bool {
        self.f.contains_key(&state) && self.f[state].contains_key(symbol)
    }

    /// to check if a complete transition is valid according to this transition function
    pub fn contains_transition(&self, state: &State, symbol: &Symbol, next_state: &State) -> bool {
        self.is_valid_transition(state, symbol) && self.f[state][symbol].contains(next_state)
    }

    /// returns the set of next_states if exists
    pub fn get_transition(&self, state: &State, symbol: &Symbol) -> Option<&HashSet<State>> {
        if self.is_valid_transition(state, symbol) {
            return Some(&self.f[state][symbol]);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_normal_and_multiple_transitions() {
        let mut nt = NTransitionFunction::new();
        let _ = nt
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));
        let _ = nt
            .add_transition(&0, &Symbol::Character('a'), &2)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        assert!(nt.f.contains_key(&0));
        assert!(nt.f[&0].contains_key(&Symbol::Character('a')));
        assert!(nt.f[&0][&Symbol::Character('a')].contains(&1));
        assert!(nt.f[&0][&Symbol::Character('a')].contains(&2));
    }

    #[test]
    fn check_adding_epsilon_transition() {
        let mut nt = NTransitionFunction::new();

        let _ = nt
            .add_transition(&0, &Symbol::Epsilon, &2)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));
        assert!(nt.f.contains_key(&0));
        assert!(nt.f[&0].contains_key(&Symbol::Epsilon));
        assert!(nt.f[&0][&Symbol::Epsilon].contains(&2));
    }

    #[test]
    fn check_adding_same_transition() {
        let mut nt = NTransitionFunction::new();
        let _ = nt
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));
        let result = nt.add_transition(&0, &Symbol::Character('a'), &1);

        assert!(result.is_err_and(|err| err.to_string().contains("Existing Transition")));
    }

    #[test]
    fn check_adding_transitions_from_another_function() {
        let mut nt1 = NTransitionFunction::new();
        let _ = nt1
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let mut nt2 = NTransitionFunction::new();
        let _ = nt2
            .add_transition(&0, &Symbol::Character('b'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let mut nt3 = NTransitionFunction::new();
        let _ = nt3
            .add_transition(&1, &Symbol::Character('b'), &4)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let mut nt4 = NTransitionFunction::new();
        let _ = nt4
            .add_transition(&5, &Symbol::Character('d'), &6)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let mut nt5 = NTransitionFunction::new();
        let _ = nt5
            .add_transition(&0, &Symbol::Character('a'), &5)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let nt = nt1.combine_transition(&nt2);
        let nt = nt.combine_transition(&nt3);
        let nt = nt.combine_transition(&nt4);
        let nt = nt.combine_transition(&nt5);

        assert!(nt.contains_transition(&0, &Symbol::Character('a'), &1));
        assert!(nt.contains_transition(&0, &Symbol::Character('a'), &5));
        assert!(nt.contains_transition(&0, &Symbol::Character('b'), &1));

        assert!(nt.contains_transition(&1, &Symbol::Character('b'), &4));

        assert!(nt.contains_transition(&5, &Symbol::Character('d'), &6));
    }
}
