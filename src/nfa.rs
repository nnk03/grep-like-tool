use std::collections::{HashSet, VecDeque};

use crate::{
    dfa::DFA,
    state::State,
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

#[allow(dead_code)]
impl NFA {
    /// getters
    pub fn num_states(&self) -> usize {
        self.num_states
    }
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    pub fn states(&self) -> &HashSet<State> {
        &self.states
    }
    pub fn start_state(&self) -> State {
        self.start_state
    }
    pub fn final_state(&self) -> State {
        self.final_state
    }

    pub fn get_transition(&self, state: &State, symbol: &Symbol) -> Option<&HashSet<State>> {
        self.transition_function.get_transition(state, symbol)
    }

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

    /// to check if a transition is valid, on a state and symbol
    pub fn is_valid_transition(&self, state: &State, symbol: &Symbol) -> bool {
        self.transition_function.is_valid_transition(state, symbol)
    }

    /// to check if a complete transition is valid according to this transition function
    pub fn contains_transition(&self, state: &State, symbol: &Symbol, next_state: &State) -> bool {
        self.transition_function
            .contains_transition(state, symbol, next_state)
    }

    /// to find out epsilon closure of a state
    pub fn epsilon_closure(&self, state: &State) -> HashSet<State> {
        let mut visited: HashSet<State> = HashSet::new();
        let mut ans = HashSet::new();
        ans.insert(*state);

        let mut q: VecDeque<State> = VecDeque::new();
        q.push_back(*state);

        while let Some(state) = q.pop_front() {
            if visited.contains(&state) {
                continue;
            }

            visited.insert(state);

            if let Some(next_states_on_epsilon) = self
                .transition_function
                .get_transition(&state, &Symbol::Epsilon)
            {
                for &next_state in next_states_on_epsilon.iter() {
                    if !visited.contains(&next_state) {
                        ans.insert(next_state);
                        q.push_back(next_state);
                    }
                }
            }
        }

        ans
    }

    /// epsilon closure of a set of states
    pub fn epsilon_closure_of_set_of_states(&self, states: &HashSet<State>) -> HashSet<State> {
        let mut ans = HashSet::new();
        if states.len() == 0 {
            return ans;
        }

        for &state in states.iter() {
            ans.insert(state);
        }

        loop {
            let mut new_states = HashSet::new();
            for &state in ans.iter() {
                let eps_closure = self.epsilon_closure(&state);
                for &state in eps_closure.iter() {
                    if !ans.contains(&state) {
                        new_states.insert(state);
                    }
                }
            }

            if new_states.len() == 0 {
                break;
            }

            for state in new_states {
                ans.insert(state);
            }
        }

        ans
    }

    /// convert a DFA to NFA
    pub fn convert_dfa_to_nfa(dfa: DFA) -> NFA {
        let mut nfa = NFA {
            num_states: dfa.num_states() + 1,
            symbol_table: dfa.symbol_table().clone(),
            states: dfa.states().clone(),
            begin_state_num: dfa.begin_state_num(),
            end_state_num: dfa.end_state_num() + 1,
            start_state: dfa.start_state(),
            final_state: dfa.num_states(),
            transition_function: NTransitionFunction::new(),
        };
        nfa.states.insert(dfa.end_state_num() + 1);
        for curr_state in dfa.begin_state_num()..=dfa.end_state_num() {
            for &symbol in dfa.symbol_table().symbols() {
                if let Some(next_state) = dfa.get_transition(&curr_state, &symbol) {
                    let _ =
                        nfa.transition_function
                            .add_transition(&curr_state, &symbol, &next_state);
                }
            }
        }

        for &final_state in dfa.final_states().iter() {
            let _ = nfa.transition_function.add_transition(
                &final_state,
                &Symbol::Epsilon,
                &nfa.final_state,
            );
        }

        nfa
    }
}

/// Functions to create an NFA from existing NFAs
impl NFA {
    /// returns NFA accepting union of 2 NFAs
    pub fn union(mut self, mut other: NFA) -> NFA {
        if self.symbol_table != other.symbol_table {
            panic!("Symbol table of 2 NFAs are not the same");
        }
        let x = self.num_states();
        let y = other.num_states();

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

    /// function to create an NFA accepting concatenation of 2 languages
    pub fn concat(mut self, mut other: NFA) -> NFA {
        if self.symbol_table != other.symbol_table {
            panic!("Symbol table of 2 NFAs are not the same");
        }

        let x = self.num_states();
        let y = other.num_states();

        let mut nfa = NFA {
            num_states: x + y + 1,
            symbol_table: self.symbol_table.clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: x + y,
            start_state: 0,
            final_state: other.final_state() + x + 1,
            transition_function: NTransitionFunction::new(),
        };
        self.extend(1);
        other.extend(x + 1);
        // insert start state
        nfa.states.insert(0);
        // final_state is same as the final_state of the second NFA
        // add states of self
        let union: HashSet<_> = nfa.states.union(&self.states).map(|&state| state).collect();
        // add states of other
        let union: HashSet<_> = union.union(&other.states).map(|&state| state).collect();

        let start_state_of_first = self.start_state();
        let final_state_of_first = self.final_state();
        let start_state_of_second = other.start_state();

        // set nfa.states to union
        nfa.states = union;

        // combine the transitions
        let new_transition_function = self
            .transition_function
            .combine_transition(&other.transition_function);
        nfa.transition_function = new_transition_function;

        // add extra transitions necessary for the concatenation function
        let epsilon = Symbol::Epsilon;

        let _ = nfa.transition_function.add_transition(
            &nfa.start_state(),
            &epsilon,
            &start_state_of_first,
        );

        let _ = nfa.transition_function.add_transition(
            &final_state_of_first,
            &epsilon,
            &start_state_of_second,
        );

        nfa
    }

    /// function to create NFA to accept Kleene star of a language
    pub fn kleene_star(mut self) -> NFA {
        let x = self.num_states();

        let mut nfa = NFA {
            num_states: x + 2,
            symbol_table: self.symbol_table.clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: x + 1,
            start_state: 0,
            final_state: x + 1,
            transition_function: NTransitionFunction::new(),
        };
        self.extend(1);

        // insert start state
        nfa.states.insert(0);
        // insert final_state
        nfa.states.insert(x + 1);

        let start_state_of_first = self.start_state();
        let final_state_of_first = self.final_state();

        let union: HashSet<_> = nfa.states.union(&self.states).map(|&state| state).collect();
        // set nfa.states to union
        nfa.states = union;

        // combine the transitions
        let new_transition_function = self.transition_function.clone();
        nfa.transition_function = new_transition_function;

        // add extra transitions necessary for the concatenation function
        let epsilon = Symbol::Epsilon;
        let _ = nfa.transition_function.add_transition(
            &nfa.start_state(),
            &epsilon,
            &start_state_of_first,
        );

        let _ = nfa.transition_function.add_transition(
            &final_state_of_first,
            &epsilon,
            &nfa.final_state(),
        );

        let _ = nfa.transition_function.add_transition(
            &nfa.final_state(),
            &epsilon,
            &nfa.start_state(),
        );

        let _ = nfa.transition_function.add_transition(
            &nfa.start_state(),
            &epsilon,
            &nfa.final_state(),
        );

        nfa
    }
}

#[cfg(test)]
mod tests {
    use crate::dfa::DFA;

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

        let dfa_union = DFA::convert_to_dfa(nfa_union);

        let result = dfa_union.run("a");
        assert!(result.is_ok_and(|res| res));

        let result = dfa_union.run("b");
        assert!(result.is_ok_and(|res| res));
    }

    #[test]
    fn check_epsilon_closure() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');

        let nfa1 = NFA::from_symbol(&a, &symbol_table);
        let nfa2 = NFA::from_symbol(&b, &symbol_table);

        let nfa_union = nfa1.union(nfa2);
        let epsilon_closure_check = nfa_union.epsilon_closure(&0);

        assert!(epsilon_closure_check.len() == 3);
        assert!(epsilon_closure_check.contains(&0));
        assert!(epsilon_closure_check.contains(&1));
        assert!(epsilon_closure_check.contains(&3));
    }

    #[test]
    fn check_conversion_to_dfa() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');

        let nfa = NFA::from_symbol(&a, &symbol_table);
        let dfa = DFA::convert_to_dfa(nfa);

        let result = dfa.run("a");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("aaa");
        assert!(result.is_ok_and(|res| !res));

        let nfa1 = NFA::from_symbol(&a, &symbol_table);
        let nfa2 = NFA::from_symbol(&b, &symbol_table);

        let nfa_union = nfa1.union(nfa2);
        let dfa = DFA::convert_to_dfa(nfa_union);

        let result = dfa.run("a");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("b");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("aaa");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_conversion_of_dfa_to_nfa() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa = DFA::from_string("abc", &symbol_table);
        let nfa = NFA::convert_dfa_to_nfa(dfa);
        let dfa = DFA::convert_to_dfa(nfa);

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_concatenation() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');

        let nfa1 = NFA::from_symbol(&a, &symbol_table);
        let nfa2 = NFA::from_symbol(&b, &symbol_table);

        let nfa_concat = nfa1.concat(nfa2);
        let dfa = DFA::convert_to_dfa(nfa_concat);

        let result = dfa.run("ab");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("aa");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_kleene_star() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');

        let a = Symbol::Character('a');

        let nfa = NFA::from_symbol(&a, &symbol_table);
        let nfa_kleene = nfa.kleene_star();

        let dfa = DFA::convert_to_dfa(nfa_kleene);

        let mut check_string = String::new();

        // check that dfa accepts epsilon string since its kleene star
        let result = dfa.run(&check_string);
        assert!(result.is_ok_and(|res| res));

        // check that dfa accepts all strings of 'a' upto length 100
        for _ in 0..100 {
            check_string.push('a');

            let result = dfa.run(&check_string);
            assert!(result.is_ok_and(|res| res));
        }

        let result = dfa.run("ab");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_sample_regular_expression() {
        // check for (a + b)*c
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');
        let c = Symbol::Character('c');

        let nfa_a = NFA::from_symbol(&a, &symbol_table);
        let nfa_b = NFA::from_symbol(&b, &symbol_table);
        let nfa_c = NFA::from_symbol(&c, &symbol_table);

        let nfa_a_plus_b = nfa_a.union(nfa_b);
        let nfa_a_plus_b_kleene_star = nfa_a_plus_b.kleene_star();

        let nfa = nfa_a_plus_b_kleene_star.concat(nfa_c);

        let dfa = DFA::convert_to_dfa(nfa);

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abbaabc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abcabc");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_intersection() {
        // check for (a + b)*c intersection abc
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');
        let c = Symbol::Character('c');

        let nfa_a = NFA::from_symbol(&a, &symbol_table);
        let nfa_b = NFA::from_symbol(&b, &symbol_table);
        let nfa_c = NFA::from_symbol(&c, &symbol_table);

        let nfa_a_plus_b = nfa_a.union(nfa_b);
        let nfa_a_plus_b_kleene_star = nfa_a_plus_b.kleene_star();

        let nfa = nfa_a_plus_b_kleene_star.concat(nfa_c);

        let dfa1 = DFA::convert_to_dfa(nfa);
        let dfa2 = DFA::from_string("abc", &symbol_table);
        let dfa = dfa1.intersection(dfa2);

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abbaabc");
        assert!(result.is_ok_and(|res| !res));

        let result = dfa.run("abcabc");
        assert!(result.is_ok_and(|res| !res));

        // now checking for (a + b)*c intersection b*c
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');

        let a = Symbol::Character('a');
        let b = Symbol::Character('b');
        let c = Symbol::Character('c');

        let nfa_a = NFA::from_symbol(&a, &symbol_table);
        let nfa_b = NFA::from_symbol(&b, &symbol_table);
        let nfa_c = NFA::from_symbol(&c, &symbol_table);

        let nfa_a_plus_b = nfa_a.union(nfa_b.clone());
        let nfa_a_plus_b_kleene_star = nfa_a_plus_b.kleene_star();

        let nfa1 = nfa_a_plus_b_kleene_star.concat(nfa_c.clone());

        let nfa_b_kleene_star = nfa_b.kleene_star();
        let nfa2 = nfa_b_kleene_star.concat(nfa_c);

        let dfa1 = DFA::convert_to_dfa(nfa1);
        let dfa2 = DFA::convert_to_dfa(nfa2);

        // this should accept b*c
        let dfa = dfa1.intersection(dfa2);
        let result = dfa.run("bc");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| !res));

        let mut check_string = String::from("c");
        let result = dfa.run(&check_string);
        assert!(result.is_ok_and(|res| res));

        // this for loop checks the acceptance of the strings bc, bbc, bbbc,....
        for _ in 0..100 {
            check_string.pop();
            check_string.push('b');
            check_string.push('c');

            let result = dfa.run(&check_string);
            assert!(result.is_ok_and(|res| res));
        }
    }
}
