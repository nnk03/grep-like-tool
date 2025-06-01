#![allow(dead_code)]

use crate::{dfa, nfa};

pub enum FA {
    DFA(dfa::DFA),
    NFA(nfa::NFA),
}
