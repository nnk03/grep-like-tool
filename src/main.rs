#![allow(dead_code)]

use std::io::{self, BufRead};

mod custom_errors;
mod d_transition_function;
mod dfa;
mod disjoint_set_union;
mod n_transition_function;
mod nfa;
mod parsing;
mod state;
mod symbol_table;
mod transition_function;
// mod finite_automaton;
// mod n_transition_function;
// mod nfa;

fn main() {
    let stdin = io::stdin();
    let mut iter = stdin.lock().lines();

    let num_test_cases = iter
        .next()
        .unwrap_or_else(|| {
            panic!("No number of test cases given");
        })
        .unwrap_or_else(|err| {
            panic!("Error in std input");
        })
        .parse::<usize>()
        .unwrap_or_else(|err| {
            panic!("Error in parsing number {}", err.to_string());
        });

    for _ in 0..num_test_cases {
        let regex = iter
            .next()
            .unwrap_or_else(|| {
                panic!("No number of test cases given");
            })
            .unwrap_or_else(|err| {
                panic!("Error in std input");
            });
        let input_string = iter
            .next()
            .unwrap_or_else(|| {
                panic!("No number of test cases given");
            })
            .unwrap_or_else(|err| {
                panic!("Error in std input");
            });

        let dfa = parsing::create_dfa_from_reg_ex(&regex);
        let dfa = match dfa {
            Ok(dfa) => dfa,
            Err(err) => {
                println!("{}", err.to_string());
                continue;
            }
        };
        let result = dfa.run(&input_string);
        match result {
            Ok(res) => {
                println!("{}", if res { "Yes" } else { "No" });
            }
            Err(err) => {
                println!("{}", err.to_string());
            }
        }
    }
}
