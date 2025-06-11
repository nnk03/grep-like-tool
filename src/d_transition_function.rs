#![allow(dead_code)]

use std::collections::HashMap;

use crate::{
    custom_errors::{AutomatonError, DFAError},
    globals::State,
    symbol_table::Symbol,
    transition_function::BasicFunctionsForTransitions,
};

#[derive(Clone, Debug)]
pub struct DTransitionFunction {
    f: HashMap<State, HashMap<Symbol, State>>,
}

impl BasicFunctionsForTransitions for DTransitionFunction {
    fn new() -> Self {
        DTransitionFunction { f: HashMap::new() }
    }

    fn add_transition(
        &mut self,
        state: &State,
        symbol: &Symbol,
        next_state: &State,
    ) -> Result<(), AutomatonError> {
        if *symbol == Symbol::Epsilon {
            return Err(AutomatonError::DFAError(DFAError::InvalidTransition(
                "Epsilon should not be present in DFA Transitions",
            )));
        }

        let entry = self.f.entry(*state).or_insert(HashMap::new());

        if entry.contains_key(symbol) {
            return Err(AutomatonError::DFAError(DFAError::InvalidTransition(
                "Adding more than one state for the same transition for DFA",
            )));
        }

        entry.insert(*symbol, *next_state);
        Ok(())
    }

    fn extend(&mut self, increment: usize) {
        let mut on_states: Vec<State> = self.f.keys().map(|&val| val).collect();

        on_states.sort();
        // going through keys in decreasing order, in order to avoid overlapping issue
        on_states.reverse();

        for &curr_state in on_states.iter() {
            if let Some((state, symbol_state_map)) = self.f.remove_entry(&curr_state) {
                let mut new_transitions: HashMap<Symbol, State> = HashMap::new();

                // (state, symbol) -> next_state
                for (symbol, next_state) in symbol_state_map.iter() {
                    new_transitions.insert(*symbol, *next_state + increment);
                }

                // update the transitions
                self.f.insert(state + increment, new_transitions);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_normal_transition() {
        let mut dt = DTransitionFunction::new();

        let _ = dt
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        assert!(dt.f.contains_key(&0));
        assert!(dt.f[&0].contains_key(&Symbol::Character('a')));
        assert_eq!(dt.f[&0][&Symbol::Character('a')], 1);
    }

    #[test]
    fn check_multiple_transition() {
        let mut dt = DTransitionFunction::new();

        let _ = dt
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        let result = dt.add_transition(&0, &Symbol::Character('a'), &2);
        assert!(result.is_err_and(|err| err.to_string().contains("Adding more than one state")));
    }

    #[test]
    fn check_adding_epsilon_transition() {
        let mut dt = DTransitionFunction::new();

        let result = dt.add_transition(&0, &Symbol::Epsilon, &2);
        assert!(result.is_err_and(|err| err.to_string().contains("Epsilon should not be present")));
    }

    #[test]
    fn check_extending() {
        let mut dt = DTransitionFunction::new();

        // (0, 'a') -> 1
        let _ = dt
            .add_transition(&0, &Symbol::Character('a'), &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        dt.extend(2);
        println!("{:#?}", dt);
        // (2, 'a') -> 3
        assert!(!dt.f.contains_key(&0));
        assert!(dt.f.contains_key(&2));

        assert!(dt.f[&2].contains_key(&Symbol::Character('a')));
        assert_eq!(dt.f[&2][&Symbol::Character('a')], 3);
    }
}
