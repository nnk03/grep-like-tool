#![allow(dead_code)]

use crate::{
    custom_errors::{AutomatonError, NFAError},
    globals::State,
    symbol_table::Symbol,
    transition_function::BasicFunctionsForTransitions,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct NTransitionFunction {
    f: HashMap<State, HashMap<Symbol, HashSet<State>>>,
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
            return Err(AutomatonError::NFAError(NFAError::ExistingTransition("")));
        }

        state_symbol_transitions.insert(*next_state);
        Ok(())
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
}
