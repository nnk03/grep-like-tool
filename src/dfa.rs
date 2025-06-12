#![allow(dead_code)]
//! This module contains the necessary functions of DFA
//!

use std::collections::HashSet;

use crate::{
    custom_errors::DFAError,
    disjoint_set_union::DSU,
    globals::State,
    symbol_table::{Symbol, SymbolTable},
    transition_function::{BasicFunctionsForTransitions, DTransitionFunction},
};

#[derive(Clone, Debug)]
pub struct DFA {
    num_states: usize,
    symbol_table: SymbolTable,
    // set of states
    states: HashSet<State>,
    // state numbers start from this
    begin_state_num: State,
    // upto end_state_num
    end_state_num: State,
    // DFA contains only a single start state
    start_state: State,
    // DFA can contain a set of final states
    final_states: HashSet<State>,
    // since indexing states by usize, we can use a Vec
    transition_function: DTransitionFunction,
}

impl DFA {
    fn from_string(s: &str, symbol_table: &SymbolTable) -> DFA {
        let num_states = s.len() + 2;
        let mut states = HashSet::new();
        let (begin_state_num, end_state_num) = (0, num_states - 1);

        for state in begin_state_num..end_state_num + 1 {
            states.insert(state);
        }

        let mut dfa = DFA {
            num_states,
            // epsilon present by default in symbol table
            symbol_table: symbol_table.clone(),
            states,
            begin_state_num,
            end_state_num,
            start_state: 0,
            final_states: HashSet::new(),
            // vector of size num_states
            transition_function: DTransitionFunction::new(),
        };

        if s.len() == 0 {
            dfa.final_states.insert(0);

            for &symbol in symbol_table.symbols() {
                match symbol {
                    Symbol::Epsilon => continue,
                    _ => {
                        dfa.transition_function
                            .add_transition(&0, &symbol, &1)
                            .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));

                        dfa.transition_function
                            .add_transition(&1, &symbol, &1)
                            .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
                    }
                }
            }

            return dfa;
        }

        let s_bytes: Vec<_> = s.as_bytes().iter().map(|&val| val as char).collect();

        let final_state = s.len();
        let reject_state = s.len() + 1;

        dfa.final_states.insert(final_state);

        for state_num in 0..s_bytes.len() {
            for &symbol in symbol_table.symbols() {
                match symbol {
                    Symbol::Epsilon => continue,
                    Symbol::Character(ch) if ch == s_bytes[state_num] => {
                        dfa.transition_function
                            .add_transition(&state_num, &Symbol::Character(ch), &(state_num + 1))
                            .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
                    }
                    Symbol::Character(ch) => {
                        dfa.transition_function
                            .add_transition(&state_num, &Symbol::Character(ch), &reject_state)
                            .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
                    }
                }
            }
        }

        dfa
    }

    pub fn run(&self, s: &str) -> Result<bool, DFAError> {
        let mut current_state = self.start_state;

        for symbol in s.as_bytes().iter().map(|&ch| Symbol::Character(ch as char)) {
            if !self.transition_function.contains_state(&current_state) {
                return Err(DFAError::InvalidState("{current_state}".to_string()));
            }

            if !self
                .transition_function
                .contains_transition(&current_state, &symbol)
            {
                return Err(DFAError::InvalidTransition(format!(
                    "Invalid Transition from {} on symbol {:?}",
                    current_state, symbol
                )));
            }

            // (current_state, symbol) -> next_state which becomes the current state
            current_state = self.transition_function[(&current_state, &symbol)];
        }

        Ok(self.final_states.contains(&current_state))
    }

    pub fn extend(&mut self, increment: usize) {
        for state in (self.begin_state_num..self.end_state_num + 1).rev() {
            if self.states.remove(&state) {
                self.states.insert(state + increment);
            }

            // if this state is present in final states, increment that too
            if self.final_states.remove(&state) {
                self.final_states.insert(state + increment);
            }
        }

        self.begin_state_num += increment;
        self.end_state_num += increment;
        self.start_state += increment;

        self.transition_function.extend(increment);
    }

    pub fn minimized_dfa(self) -> DFA {
        let dfa = self;
        let n = dfa.num_states;
        let offset = dfa.begin_state_num;
        let mut marked: Vec<Vec<bool>> = vec![vec![false; n]; n];

        for first_state in dfa.begin_state_num..=dfa.end_state_num {
            for second_state in dfa.begin_state_num..=dfa.end_state_num {
                if first_state == second_state {
                    continue;
                }

                if dfa.final_states.contains(&first_state)
                    && !dfa.final_states.contains(&second_state)
                {
                    marked[first_state - offset][second_state - offset] = true;
                    marked[second_state - offset][first_state - offset] = true;
                }
            }
        }

        let mut is_changed = true;

        while is_changed {
            is_changed = false;

            for first_state in dfa.begin_state_num..=dfa.end_state_num {
                for second_state in dfa.begin_state_num..=dfa.end_state_num {
                    if marked[first_state - offset][second_state - offset] {
                        continue;
                    }

                    for symbol in dfa.symbol_table.symbols() {
                        let does_both_have_transition_on_symbol = dfa
                            .transition_function
                            .contains_transition(&first_state, symbol)
                            && dfa
                                .transition_function
                                .contains_transition(&second_state, symbol);

                        // if both have transition on the same symbol
                        // and the pair (next_of_first_state, next_of_second_state) is marked
                        // then mark this pair
                        if does_both_have_transition_on_symbol {
                            let (next_of_first_state, next_of_second_state) = (
                                dfa.transition_function[(&first_state, symbol)],
                                dfa.transition_function[(&second_state, symbol)],
                            );

                            if marked[next_of_first_state - offset][next_of_second_state - offset]
                                && !marked[first_state - offset][second_state - offset]
                            {
                                marked[first_state - offset][second_state - offset] = true;
                                marked[second_state - offset][first_state - offset] = true;
                                is_changed = true;
                            }
                        }
                    }
                }
            }
        }

        let mut dsu = DSU::new(dfa.num_states);
        for first_state in dfa.begin_state_num..=dfa.end_state_num {
            for second_state in dfa.begin_state_num..=dfa.end_state_num {
                if first_state == second_state {
                    continue;
                }

                if !marked[first_state - offset][second_state - offset] {
                    // then this pair is indistinguishable, i.e it can be merged
                    dsu.union(first_state - offset, second_state - offset);
                }
            }
        }

        let state_representative_map = dsu.state_representative_map(offset);
        let minimum_dfa_len = state_representative_map.len();
        let mut new_dfa = DFA {
            num_states: state_representative_map.len(),
            symbol_table: dfa.symbol_table.clone(),
            states: HashSet::from_iter(dfa.begin_state_num..dfa.begin_state_num + minimum_dfa_len),
            begin_state_num: dfa.begin_state_num,
            end_state_num: dfa.begin_state_num + minimum_dfa_len - 1,
            start_state: state_representative_map[&dfa.begin_state_num],
            final_states: HashSet::from_iter(
                dfa.final_states
                    .iter()
                    .map(|&state| state_representative_map[&state]),
            ),
            transition_function: DTransitionFunction::new(),
        };

        for (curr_state, symbol_to_next_state_map) in dfa.transition_function.f {
            if state_representative_map[&curr_state] != curr_state {
                // this is not present in minimum dfa
                continue;
            }

            for (symbol, next_state) in symbol_to_next_state_map {
                // curr_state is present in minimum dfa
                new_dfa
                    .transition_function
                    .add_transition(&curr_state, &symbol, &state_representative_map[&next_state])
                    .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
            }
        }

        new_dfa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_acceptance_of_dfa_constructed_from_string() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa = DFA::from_string("abc", &symbol_table);

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_acceptance_of_dfa_for_empty_string() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa = DFA::from_string("", &symbol_table);

        let result = dfa.run("");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_invalid_transition() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let dfa = DFA::from_string("abc", &symbol_table);

        let result = dfa.run("abc");
        assert!(result.is_err_and(|res| res.to_string().contains("Invalid Transition")));
    }

    #[test]
    fn check_extending() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let mut dfa = DFA::from_string("abc", &symbol_table);

        dfa.extend(2);

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));

        for state in 0..=1 {
            assert!(!dfa.states.contains(&state));
        }

        for state in 2..=6 {
            assert!(dfa.states.contains(&state));
        }
    }

    #[test]
    fn check_minimization() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa = DFA::from_string("abc", &symbol_table);
        let dfa = dfa.minimized_dfa();

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        // let result = dfa.run("abd");
        // assert!(result.is_ok_and(|res| !res));
    }
}
