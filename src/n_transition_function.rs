#![allow(dead_code)]

use crate::{
    custom_errors::DFAError,
    globals::State,
    symbol_table::{Symbol, SymbolTable},
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct NTransitionFunction {
    f: HashMap<State, HashMap<Symbol, HashSet<State>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
