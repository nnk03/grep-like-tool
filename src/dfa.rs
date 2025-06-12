#![allow(dead_code)]
//! This module contains the necessary functions of DFA
//!

use std::collections::{HashMap, HashSet};

use crate::{
    custom_errors::DFAError,
    globals::State,
    symbol_table::{Symbol, SymbolTable},
    transition_function::{BasicFunctionsForTransitions, DTransitionFunction, TransitionFunction},
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

        for state_num in dfa.begin_state_num..=dfa.end_state_num {
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
}
