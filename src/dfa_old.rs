#![allow(dead_code)]
//! This module contains the necessary functions of DFA
//!

use std::collections::{HashMap, HashSet};

use crate::{
    custom_errors::DFAError,
    globals::State,
    symbol_table::{Symbol, SymbolTable},
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
    transition_function: Vec<HashMap<Symbol, State>>,
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
            transition_function: vec![HashMap::new(); num_states],
        };

        if s.len() == 0 {
            dfa.final_states.insert(0);
            dfa.transition_function.insert(0, HashMap::new());
            dfa.transition_function.insert(1, HashMap::new());

            for &symbol in symbol_table.symbols() {
                match symbol {
                    // dfa does not contain epsilon transitions
                    Symbol::Epsilon => continue,
                    _ => {
                        dfa.transition_function[0].insert(symbol, 1);
                        dfa.transition_function[1].insert(symbol, 1);
                    }
                }
            }

            return dfa;
        }

        let s_bytes: Vec<_> = s.as_bytes().iter().map(|&val| val as char).collect();

        let final_state = s.len();
        let reject_state = s.len() + 1;

        dfa.final_states.insert(final_state);

        for state_num in 0..s.len() {
            for &symbol in symbol_table.symbols() {
                match symbol {
                    Symbol::Epsilon => continue,
                    Symbol::Character(ch) if ch == s_bytes[state_num] => {
                        dfa.transition_function[state_num]
                            .insert(Symbol::Character(ch), state_num + 1);
                    }
                    Symbol::Character(ch) => {
                        dfa.transition_function[state_num]
                            .insert(Symbol::Character(ch), reject_state);
                    }
                }
            }
        }

        for &symbol in symbol_table.symbols() {
            match symbol {
                // dfa does not contain epsilon transitions
                Symbol::Epsilon => continue,
                _ => {
                    dfa.transition_function[final_state].insert(symbol, reject_state);
                    dfa.transition_function[reject_state].insert(symbol, reject_state);
                }
            }
        }

        dfa
    }

    pub fn run(&self, s: &str) -> Result<bool, DFAError> {
        let mut current_state = self.start_state;

        for symbol in s.as_bytes().iter().map(|&ch| Symbol::Character(ch as char)) {
            if current_state >= self.transition_function.len() {
                return Err(DFAError::InvalidState("{current_state}".to_string()));
            }

            if !self.transition_function[current_state].contains_key(&symbol) {
                return Err(DFAError::InvalidTransition(format!(
                    "Invalid Transition from {} on symbol {:?}",
                    current_state, symbol
                )));
            }

            current_state = self.transition_function[current_state][&symbol];
        }

        Ok(self.final_states.contains(&current_state))
    }

    pub fn extend(&mut self, increment: usize) {
        let mut dfa = self.clone();

        for state in (self.begin_state_num..self.end_state_num + 1).rev() {
            dfa.states.remove(&state);
            dfa.states.insert(state + increment);

            // if this state is present in final states, increment that too
            if dfa.final_states.remove(&state) {
                dfa.final_states.insert(state + increment);
            }
        }
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
}
