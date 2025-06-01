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

    /// add symbol is for inserting characters other than EPSILON
    pub fn add_symbol(&mut self, ch: char) {
        if self.symbol_to_number.contains_key(&Symbol::Character(ch)) {
            return;
        }

        // start inserting with 1
        self.symbol_to_number
            .insert(Symbol::Character(ch), self.current_number);
        self.number_to_symbol
            .insert(self.current_number, Symbol::Character(ch));

        self.current_number += 1;
    }

    /// returns the number of symbols present
    pub fn len(&self) -> usize {
        self.symbol_to_number.len()
    }

    pub fn symbols(&self) -> Keys<'_, Symbol, usize> {
        self.symbol_to_number.keys()
    }
}
