use std::collections::HashSet;

use thiserror::Error;

use crate::{
    dfa::DFA,
    nfa::NFA,
    symbol_table::{Symbol, SymbolTable},
};

type Stack<T> = Vec<T>;

#[derive(Clone, Debug, Error)]
pub enum ParsingError {
    #[error("Parsing Error")]
    ParseError,
}

/// creating an NFA from reg-ex
pub fn create_nfa_from_reg_ex(input: &str) -> Result<NFA, ParsingError> {
    let symbol_table = create_symbol_table(input)?;

    let bytes = input.as_bytes();

    let mut string_stack: Stack<&str> = Stack::new();
    let mut nfa_stack: Stack<NFA> = Stack::new();

    let mut i = 0;
    let n = bytes.len();
    while i < n {
        if bytes[i] == b'c' {
            // has to start with concat
            if i + 7 >= n {
                return Err(ParsingError::ParseError);
            }
            if &input[i..i + 7] == "concat(" {
                string_stack.push("(");
                string_stack.push("concat");
                i += 7;
            } else {
                return Err(ParsingError::ParseError);
            }
        } else if bytes[i] == b'u' {
            // has to be union
            if i + 6 >= n {
                return Err(ParsingError::ParseError);
            }

            if &input[i..i + 6] == "union(" {
                string_stack.push("(");
                string_stack.push("union");
                i += 6;
            } else {
                return Err(ParsingError::ParseError);
            }
        } else if bytes[i] == b's' {
            // must be star or symbol
            if i + 5 >= n {
                return Err(ParsingError::ParseError);
            }

            if &input[i..i + 5] == "star(" {
                string_stack.push("(");
                string_stack.push("star");
                i += 5;
            } else if i + 7 >= n {
                return Err(ParsingError::ParseError);
            } else if &input[i..i + 7] == "symbol(" && bytes[i + 8] == b')' {
                // since its a symbol it will be only a single character
                // skip by length of symbol(a)
                let nfa_from_symbol =
                    NFA::from_symbol(&Symbol::Character(bytes[i + 7] as char), &symbol_table);
                nfa_stack.push(nfa_from_symbol);

                i += 9;
            } else {
                return Err(ParsingError::ParseError);
            }
        } else if bytes[i] == b')' {
            i += 1;

            while let Some(string) = string_stack.pop() {
                match string {
                    "star" => {
                        if let Some(nfa) = nfa_stack.pop() {
                            // push kleene star onto stack
                            let nfa_kleene_star = nfa.kleene_star();
                            nfa_stack.push(nfa_kleene_star);
                        } else {
                            return Err(ParsingError::ParseError);
                        }
                    }
                    "union" => {
                        if nfa_stack.len() < 2 {
                            return Err(ParsingError::ParseError);
                        }
                        let second_nfa = nfa_stack.pop().unwrap();
                        let first_nfa = nfa_stack.pop().unwrap();
                        let nfa_union = first_nfa.union(second_nfa);

                        nfa_stack.push(nfa_union);
                    }
                    "concat" => {
                        if nfa_stack.len() < 2 {
                            return Err(ParsingError::ParseError);
                        }
                        let second_nfa = nfa_stack.pop().unwrap();
                        let first_nfa = nfa_stack.pop().unwrap();
                        let nfa_concat = first_nfa.concat(second_nfa);

                        nfa_stack.push(nfa_concat);
                    }
                    "(" => {
                        break;
                    }
                    _ => {
                        return Err(ParsingError::ParseError);
                    }
                }
            }
        } else if bytes[i] == b',' {
            // comma is just a separator
            i += 1;
        } else {
            return Err(ParsingError::ParseError);
        }
    }

    if nfa_stack.len() != 1 {
        return Err(ParsingError::ParseError);
    }

    Ok(nfa_stack.pop().unwrap())
}

/// creating a DFA from reg-ex
pub fn create_dfa_from_reg_ex(input: &str) -> Result<DFA, ParsingError> {
    let nfa = create_nfa_from_reg_ex(input)?;
    let dfa = DFA::convert_to_dfa(nfa);
    let dfa = dfa.minimized_dfa();

    Ok(dfa)
}

/// function to extract the symbols from the input string
fn extract_symbols(input: &str) -> Result<HashSet<char>, ParsingError> {
    let mut result = HashSet::new();
    let bytes = input.as_bytes();

    let mut i = 0;
    while i + 8 < bytes.len() {
        if &input[i..i + 7] == "symbol(" && bytes[i + 8] != b')' {
            return Err(ParsingError::ParseError);
        }
        if &input[i..i + 7] == "symbol(" && bytes[i + 8] == b')' {
            // The character at position i+7 is the one inside symbol(...)
            result.insert(input.chars().nth(i + 7).unwrap());
            i += 9; // move past "symbol(x)"
        } else {
            i += 1;
        }
    }

    Ok(result)
}

/// function to create a symbol table after extracting the symbols from the input reg ex
fn create_symbol_table(input: &str) -> Result<SymbolTable, ParsingError> {
    let symbols = extract_symbols(input)?;

    let mut symbol_table = SymbolTable::new();

    for character in symbols {
        symbol_table.add_character(character);
    }

    Ok(symbol_table)
}

#[cfg(test)]
mod tests {
    use crate::dfa::DFA;

    use super::*;

    #[test]
    fn check_extracting_symbols() {
        let input = "concat(concat(symbol(a),symbol(1)),star(union(symbol(0),symbol(1))))";
        let symbols = extract_symbols(input).unwrap();
        assert!(symbols.contains(&'a'));
        assert!(symbols.contains(&'0'));
        assert!(symbols.contains(&'1'));
    }

    #[test]
    fn check_dfa() {
        let input = "star(symbol(a))";
        let nfa = create_nfa_from_reg_ex(input).unwrap();

        let dfa = DFA::convert_to_dfa(nfa);
        let result = dfa.run("aaaa");
        assert!(result.is_ok_and(|res| res));

        let input = "concat(concat(symbol(0),symbol(1)),star(union(symbol(0),symbol(1))))";
        let nfa = create_nfa_from_reg_ex(input).unwrap();

        let dfa = DFA::convert_to_dfa(nfa);
        let result = dfa.run("1011");
        assert!(result.is_ok_and(|res| !res));
        let result = dfa.run("01");
        assert!(result.is_ok_and(|res| res));
        let result = dfa.run("010011");
        assert!(result.is_ok_and(|res| res));
    }
}
