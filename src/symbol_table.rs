#![allow(dead_code)]
//! This module contains the necessary functions for Symbol and SymbolTable

use std::{
    collections::{hash_map::Keys, HashMap},
    ops::Index,
};

/// special value for EPSILON
const EPSILON_VALUE: usize = 0;

/// Type for Symbols
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Symbol {
    Epsilon,
    Character(char),
}

/// Struct to hold the symbols and their corresponding numbers
#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbol_to_number: HashMap<Symbol, usize>,
    number_to_symbol: HashMap<usize, Symbol>,
    current_number: usize,
}

impl Index<Symbol> for SymbolTable {
    type Output = usize;

    /// returns the number of corresponding symbol when indexed with Symbol
    fn index(&self, index: Symbol) -> &Self::Output {
        return &self.symbol_to_number[&index];
    }
}

impl Index<usize> for SymbolTable {
    type Output = Symbol;

    /// returns the Symbol for the corresponding number, when indexed with usize
    fn index(&self, index: usize) -> &Self::Output {
        return &self.number_to_symbol[&index];
    }
}

impl SymbolTable {
    /// returns a new instance of symbol table initialised with Symbol::Epsilon
    pub fn new() -> SymbolTable {
        let mut symbol_table = SymbolTable {
            symbol_to_number: HashMap::new(),
            number_to_symbol: HashMap::new(),
            // 0 is reserved for EPSILON
            current_number: EPSILON_VALUE + 1,
        };

        symbol_table
            .symbol_to_number
            .insert(Symbol::Epsilon, EPSILON_VALUE);

        symbol_table
            .number_to_symbol
            .insert(EPSILON_VALUE, Symbol::Epsilon);

        symbol_table
    }

    /// add_symbol is for adding symbol
    pub fn add_symbol(&mut self, symbol: Symbol) {
        if symbol == Symbol::Epsilon {
            return;
        }

        if self.symbol_to_number.contains_key(&symbol) {
            return;
        }

        // start inserting with 1
        self.symbol_to_number.insert(symbol, self.current_number);
        self.number_to_symbol.insert(self.current_number, symbol);

        self.current_number += 1;
    }

    /// add character is for inserting characters other than EPSILON
    pub fn add_character(&mut self, ch: char) {
        self.add_symbol(Symbol::Character(ch));
    }

    /// returns the number of symbols present
    pub fn len(&self) -> usize {
        self.symbol_to_number.len()
    }

    pub fn symbols(&self) -> Keys<'_, Symbol, usize> {
        self.symbol_to_number.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epsilon_present() {
        let st = SymbolTable::new();
        assert!(st.symbol_to_number.contains_key(&Symbol::Epsilon));
        assert_eq!(0, st.symbol_to_number[&Symbol::Epsilon]);
    }

    #[test]
    fn test_adding_symbol() {
        let mut st = SymbolTable::new();
        st.add_character('c');

        assert!(st.symbol_to_number.contains_key(&Symbol::Character('c')));
        assert_eq!(st.symbol_to_number[&Symbol::Character('c')], 1);
        assert_eq!(st.number_to_symbol[&1], Symbol::Character('c'));

        assert_eq!(st.len(), 2);
    }
}
