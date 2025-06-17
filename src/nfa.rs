#![allow(dead_code)]

use std::collections::HashSet;

use crate::{
    globals::State,
    symbol_table::{Symbol, SymbolTable},
    transition_function::{BasicFunctionsForTransitions, NTransitionFunction},
};

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
    // any NFA can be modelled as an NFA with a single start and end state
    start_state: State,
    final_state: State,
    transition_function: NTransitionFunction,
}

impl NFA {
    /// creates an NFA which accepts a single symbol
    pub fn from_symbol(symbol: &Symbol, symbol_table: &SymbolTable) -> NFA {
        if *symbol == Symbol::Epsilon {
            let mut nfa = NFA {
                num_states: 1,
                symbol_table: symbol_table.clone(),
                states: HashSet::new(),
                begin_state_num: 0,
                end_state_num: 0,
                start_state: 0,
                final_state: 0,
                transition_function: NTransitionFunction::new(),
            };

            nfa.states.insert(0);

            return nfa;
        }

        let mut nfa = NFA {
            num_states: 2,
            symbol_table: symbol_table.clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: 1,
            start_state: 0,
            final_state: 1,
            transition_function: NTransitionFunction::new(),
        };

        nfa.states.insert(0);
        nfa.states.insert(1);

        nfa.transition_function
            .add_transition(&0, symbol, &1)
            .unwrap_or_else(|err| panic!("Error in adding transition : {}", err.to_string()));

        nfa
    }

    /// extending by `increment`
    pub fn extend(&mut self, increment: usize) {
        for state in (self.begin_state_num..self.end_state_num + 1).rev() {
            if self.states.remove(&state) {
                self.states.insert(state + increment);
            }

            if self.final_state == state {
                self.final_state += increment;
            }
        }

        self.begin_state_num += increment;
        self.end_state_num += increment;
        self.start_state += increment;

        self.transition_function.extend(increment);
    }

    pub fn union(mut self, mut other: NFA) -> NFA {
        if self.symbol_table != other.symbol_table {
            panic!("Symbol table of 2 NFAs are not the same");
        }
        let x = self.num_states;
        let y = other.num_states;

        let mut nfa = NFA {
            num_states: x + y + 2,
            symbol_table: self.symbol_table.clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: x + y + 1,
            start_state: 0,
            final_state: x + y + 1,
            transition_function: NTransitionFunction::new(),
        };

        self.extend(1);
        other.extend(x + 1);

        // add start and final state
        nfa.states.insert(0);
        nfa.states.insert(x + y + 1);

        // add states of self
        let union: HashSet<_> = nfa.states.union(&self.states).map(|&state| state).collect();
        // add states of other
        let union: HashSet<_> = union.union(&other.states).map(|&state| state).collect();

        // set nfa.states to union
        nfa.states = union;

        // combine the transitions
        let new_transition_function = self
            .transition_function
            .combine_transition(&other.transition_function);
        nfa.transition_function = new_transition_function;

        // add extra transitions necessary for the union function
        let epsilon = Symbol::Epsilon;
        let _ = nfa.transition_function.add_transition(&0, &epsilon, &1);
        let _ = nfa
            .transition_function
            .add_transition(&0, &epsilon, &(x + 1));
        let _ = nfa
            .transition_function
            .add_transition(&x, &epsilon, &(x + y + 1));
        let _ = nfa
            .transition_function
            .add_transition(&(x + y), &epsilon, &(x + y + 1));

        nfa
    }

    /// to check if a transition is valid, on a state and symbol
    pub fn is_valid_transition(&self, state: &State, symbol: &Symbol) -> bool {
        self.transition_function.is_valid_transition(state, symbol)
    }

    /// to check if a complete transition is valid according to this transition function
    pub fn contains_transition(&self, state: &State, symbol: &Symbol, next_state: &State) -> bool {
        self.transition_function
            .contains_transition(state, symbol, next_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_symbol_nfa() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');

        let nfa = NFA::from_symbol(&Symbol::Character('a'), &symbol_table);
        assert!(nfa.contains_transition(&0, &Symbol::Character('a'), &1));

        let nfa = NFA::from_symbol(&Symbol::Epsilon, &symbol_table);

        // nfa transitions are empty
        let transition_keys: Vec<_> = nfa.transition_function.f.keys().collect();
        assert_eq!(transition_keys, Vec::<&State>::new());
    }

    #[test]
    fn check_union_of_two_nfas() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let epsilon = Symbol::Epsilon;
        let a = Symbol::Character('a');
        let b = Symbol::Character('b');

        let nfa1 = NFA::from_symbol(&a, &symbol_table);
        let nfa2 = NFA::from_symbol(&b, &symbol_table);

        let nfa_union = nfa1.union(nfa2);

        assert!(nfa_union.contains_transition(&0, &epsilon, &1));
        assert!(nfa_union.contains_transition(&0, &epsilon, &3));
        assert!(nfa_union.contains_transition(&1, &a, &2));
        assert!(nfa_union.contains_transition(&3, &b, &4));
        assert!(nfa_union.contains_transition(&2, &epsilon, &5));
        assert!(nfa_union.contains_transition(&4, &epsilon, &5));
    }
}
