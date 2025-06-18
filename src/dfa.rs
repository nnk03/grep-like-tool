#![allow(dead_code)]
//! This module contains the necessary functions of DFA
//!

use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    custom_errors::DFAError,
    disjoint_set_union::DSU,
    nfa::NFA,
    state::{State, StateSet},
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

/// getters
impl DFA {
    pub fn num_states(&self) -> usize {
        self.num_states
    }
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    pub fn begin_state_num(&self) -> State {
        self.begin_state_num
    }
    pub fn end_state_num(&self) -> State {
        self.end_state_num
    }
    pub fn start_state(&self) -> State {
        self.start_state
    }
    pub fn final_states(&self) -> &HashSet<State> {
        &self.final_states
    }
    pub fn states(&self) -> &HashSet<State> {
        &self.states
    }
}

impl DFA {
    /// create a DFA from a string
    pub fn from_string(s: &str, symbol_table: &SymbolTable) -> DFA {
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

        for &symbol in symbol_table.symbols() {
            match symbol {
                Symbol::Epsilon => continue,
                Symbol::Character(ch) => {
                    dfa.transition_function
                        .add_transition(&final_state, &Symbol::Character(ch), &reject_state)
                        .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
                    dfa.transition_function
                        .add_transition(&reject_state, &Symbol::Character(ch), &reject_state)
                        .unwrap_or_else(|err| panic!("{}", format!("{}", err.to_string())));
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
                .is_valid_transition(&current_state, &symbol)
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

    /// extending by `increment`
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
        let mut dfa = self;

        // cleanup dfa before minimizing
        dfa.cleanup();

        let n = dfa.num_states;
        let offset = dfa.begin_state_num;
        let mut marked: Vec<Vec<bool>> = vec![vec![false; n]; n];

        for first_state in dfa.begin_state_num..=dfa.end_state_num {
            for second_state in first_state + 1..=dfa.end_state_num {
                // first_state < second_state

                if dfa.final_states.contains(&first_state)
                    && !dfa.final_states.contains(&second_state)
                {
                    // first index always less than second index
                    marked[first_state - offset][second_state - offset] = true;
                } else if !dfa.final_states.contains(&first_state)
                    && dfa.final_states.contains(&second_state)
                {
                    // first index always less than second index
                    marked[first_state - offset][second_state - offset] = true;
                }
            }
        }

        loop {
            let mut is_changed = false;

            for first_state in dfa.begin_state_num..=dfa.end_state_num {
                for second_state in first_state + 1..=dfa.end_state_num {
                    if marked[first_state - offset][second_state - offset] {
                        continue;
                    }

                    for symbol in dfa.symbol_table.symbols() {
                        if *symbol == Symbol::Epsilon {
                            // there will be no transition for epsilon symbol
                            continue;
                        }

                        // if both have transition on the same symbol
                        // and the pair (next_of_first_state, next_of_second_state) is marked
                        // then mark this pair
                        // since this is a DFA, it must have transition on same symbol
                        let (next_of_first_state, next_of_second_state) = (
                            dfa.transition_function[(&first_state, symbol)],
                            dfa.transition_function[(&second_state, symbol)],
                        );

                        // since we are marking with the convention first_index < second_index
                        let (next_of_first_state, next_of_second_state) = (
                            next_of_first_state.min(next_of_second_state),
                            next_of_first_state.max(next_of_second_state),
                        );

                        if marked[next_of_first_state - offset][next_of_second_state - offset]
                            && !marked[first_state - offset][second_state - offset]
                        {
                            marked[first_state - offset][second_state - offset] = true;
                            is_changed = true;
                        }
                    }
                }
            }

            if !is_changed {
                break;
            }
        }

        let mut dsu = DSU::new(dfa.num_states);
        for first_state in dfa.begin_state_num..=dfa.end_state_num {
            for second_state in first_state + 1..=dfa.end_state_num {
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

    /// cleanup the dfa by removing inaccessible states and ordering the states
    pub fn cleanup(&mut self) {
        let mut q: VecDeque<State> = VecDeque::new();
        q.push_back(0);
        let mut visited: HashSet<State> = HashSet::new();

        // find the reachable states using BFS
        while let Some(state) = q.pop_front() {
            if visited.contains(&state) {
                continue;
            }

            visited.insert(state);

            for &symbol in self.symbol_table.symbols() {
                if let Some(next_state) = self.get_transition(&state, &symbol) {
                    if !visited.contains(&next_state) {
                        q.push_back(next_state);
                    }
                }
            }
        }

        let mut existing_state_to_new_state_map: HashMap<State, State> = HashMap::new();

        existing_state_to_new_state_map.insert(self.start_state, 0);

        let mut curr_state_num = 1;
        for &state in visited.iter() {
            if state == self.start_state {
                continue;
            }

            existing_state_to_new_state_map.insert(state, curr_state_num);
            curr_state_num += 1;
        }

        let mut transition_function = DTransitionFunction::new();

        for (&state, symbol_to_next_state) in self.transition_function.f.iter() {
            // only perform the action for reachable states
            if !existing_state_to_new_state_map.contains_key(&state) {
                continue;
            }

            for (&symbol, &next_state) in symbol_to_next_state.iter() {
                // only perform the action for reachable states
                if !existing_state_to_new_state_map.contains_key(&next_state) {
                    continue;
                }

                transition_function
                    .add_transition(
                        &existing_state_to_new_state_map[&state],
                        &symbol,
                        &existing_state_to_new_state_map[&next_state],
                    )
                    .unwrap_or_else(|err| {
                        panic!("Error in adding transition : {}", err.to_string())
                    });
            }
        }

        // symbol table remains unchanged
        let num_states = visited.len();
        self.num_states = num_states;
        self.states = HashSet::from_iter((0..num_states).into_iter());

        self.begin_state_num = 0;
        self.end_state_num = num_states - 1;

        self.transition_function.f.clear();
        self.transition_function = transition_function;

        self.start_state = 0;
        self.final_states = self
            .final_states
            .iter()
            .filter_map(|state| {
                if !existing_state_to_new_state_map.contains_key(state) {
                    None
                } else {
                    Some(existing_state_to_new_state_map[state])
                }
            })
            .collect();
    }

    /// get transition if it is valid
    pub fn get_transition(&self, state: &State, symbol: &Symbol) -> Option<State> {
        if self.transition_function.is_valid_transition(state, symbol) {
            return Some(self.transition_function[(state, symbol)]);
        }

        None
    }

    /// to get the subsets of a collection
    fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        (0..2usize.pow(s.len() as u32))
            .map(|i| {
                s.iter()
                    .enumerate()
                    .filter(|&(t, _)| ((i >> t) & 1) == 1)
                    .map(|(_, element)| element.clone())
                    .collect()
            })
            .collect()
    }

    /// converting NFA to a minimized DFA
    pub fn convert_to_dfa(nfa: NFA) -> DFA {
        let mut curr_state_num = 0;
        let mut subset_to_num_map: HashMap<StateSet, State> = HashMap::new();
        let mut num_to_subset_map: HashMap<State, StateSet> = HashMap::new();

        let mut get_state_equivalent_number = |subset_state| -> State {
            if subset_to_num_map.contains_key(&subset_state) {
                return subset_to_num_map[&subset_state];
            }
            subset_to_num_map.insert(subset_state.clone(), curr_state_num);
            num_to_subset_map.insert(curr_state_num, subset_state.clone());
            curr_state_num += 1;

            subset_to_num_map[&subset_state]
        };

        let start_state_closure = StateSet::new(nfa.epsilon_closure(&nfa.start_state()));

        let mut q: VecDeque<StateSet> = VecDeque::new();
        q.push_back(start_state_closure);

        let mut dfa = DFA {
            num_states: 2_u32.pow(nfa.num_states() as u32) as usize,
            symbol_table: nfa.symbol_table().clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: 0,
            start_state: 0,
            final_states: HashSet::new(),
            transition_function: DTransitionFunction::new(),
        };

        let mut visited: HashSet<State> = HashSet::new();

        while let Some(curr_set_of_states) = q.pop_front() {
            if visited.contains(&get_state_equivalent_number(curr_set_of_states.clone())) {
                continue;
            }
            visited.insert(get_state_equivalent_number(curr_set_of_states.clone()));

            let curr_state_number = get_state_equivalent_number(curr_set_of_states.clone());
            dfa.states.insert(curr_state_number);

            // if the set of states contains the accept state of nfa
            // it means that there exists some path in the NFA that reaches the final state
            // hence add this as accept state of dfa
            if curr_set_of_states.states().contains(&nfa.final_state()) {
                dfa.final_states.insert(curr_state_number);
            }

            for &symbol in dfa.symbol_table.symbols() {
                if symbol == Symbol::Epsilon {
                    continue;
                }

                let mut next_states_on_this_symbol = HashSet::new();

                for &state in curr_set_of_states.states() {
                    if let Some(next_state_set) = nfa.get_transition(&state, &symbol) {
                        for &next_state in next_state_set.iter() {
                            next_states_on_this_symbol.insert(next_state);
                        }
                    }
                }

                let next_states_on_this_symbol =
                    nfa.epsilon_closure_of_set_of_states(&next_states_on_this_symbol);

                let next_states_on_this_symbol = StateSet::new(next_states_on_this_symbol);

                let next_state_number =
                    get_state_equivalent_number(next_states_on_this_symbol.clone());

                let _ = dfa.transition_function.add_transition(
                    &curr_state_number,
                    &symbol,
                    &next_state_number,
                );

                if !visited.contains(&next_state_number) {
                    q.push_back(next_states_on_this_symbol);
                }
            }
        }

        dfa.num_states = visited.len();
        dfa.end_state_num = visited.len() - 1;
        dfa.states = visited;

        // minimize the dfa
        let dfa = dfa.minimized_dfa();

        dfa
    }
}

impl DFA {
    /// function for complement of a DFAs
    pub fn complement(&self) -> DFA {
        let mut dfa = self.clone();
        dfa.final_states.clear();

        for state in self.states.iter() {
            if !self.final_states.contains(state) {
                dfa.final_states.insert(*state);
            }
        }

        let dfa = dfa.minimized_dfa();
        dfa
    }

    /// function for intersection of 2 DFAs
    pub fn intersection(&self, other: DFA) -> DFA {
        if self.symbol_table != other.symbol_table {
            panic!("Symbol table of 2 NFAs are not the same");
        }
        let x = self.num_states();
        let y = other.num_states();

        let mut dfa = DFA {
            num_states: x * y,
            symbol_table: self.symbol_table.clone(),
            states: HashSet::new(),
            begin_state_num: 0,
            end_state_num: x * y - 1,
            start_state: 0,
            final_states: HashSet::new(),
            transition_function: DTransitionFunction::new(),
        };

        let mut curr_state_num: State = 0;
        let mut pair_to_state_number: HashMap<(State, State), State> = HashMap::new();

        pair_to_state_number.insert((self.start_state(), other.start_state()), curr_state_num);
        curr_state_num += 1;

        for first_state in self.begin_state_num()..=self.end_state_num() {
            for second_state in other.begin_state_num()..=other.end_state_num() {
                let pair = (first_state, second_state);

                if !pair_to_state_number.contains_key(&pair) {
                    pair_to_state_number.insert(pair, curr_state_num);
                    curr_state_num += 1;
                }

                if self.final_states().contains(&first_state)
                    && other.final_states().contains(&second_state)
                {
                    // if both are in final states of respective machines, add that to final state
                    // of the resultant dfa
                    dfa.final_states.insert(pair_to_state_number[&pair]);
                }
            }
        }

        for first_state in self.begin_state_num()..=self.end_state_num() {
            for second_state in other.begin_state_num()..=other.end_state_num() {
                let pair = (first_state, second_state);
                let state = pair_to_state_number[&pair];

                for symbol in dfa.symbol_table.symbols() {
                    if *symbol == Symbol::Epsilon {
                        continue;
                    }

                    // transition will be always valid since it is a DFA
                    // but there was a test case in which this was invalid
                    // rectified it with check if it is none
                    // but need to check it once
                    let next_state_pair = (
                        self.get_transition(&first_state, symbol),
                        other.get_transition(&second_state, symbol),
                    );

                    if next_state_pair.0.is_none() || next_state_pair.1.is_none() {
                        continue;
                    }
                    let next_state_pair = (next_state_pair.0.unwrap(), next_state_pair.1.unwrap());

                    let next_state = pair_to_state_number[&next_state_pair];

                    let _ = dfa
                        .transition_function
                        .add_transition(&state, symbol, &next_state);
                }
            }
        }

        let dfa = dfa.minimized_dfa();
        dfa
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

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_minimization_single_alphabet() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        let dfa = DFA::from_string("aa", &symbol_table);

        let dfa = dfa.minimized_dfa();

        let result = dfa.run("aa");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("aaa");
        assert!(result.is_ok_and(|res| !res));
    }

    #[test]
    fn check_minimization_empty_string() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');

        let dfa = DFA::from_string("", &symbol_table);
        let dfa = dfa.minimized_dfa();

        let result = dfa.run("");
        assert!(result.is_ok_and(|res| res));
    }

    #[test]
    fn check_complementation_of_dfa() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa = DFA::from_string("abc", &symbol_table);
        let dfa = dfa.complement();

        let result = dfa.run("abc");
        assert!(result.is_ok_and(|res| !res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| res));
    }

    #[test]
    fn check_simple_intersection_of_dfa() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_character('a');
        symbol_table.add_character('b');
        symbol_table.add_character('c');
        symbol_table.add_character('d');

        let dfa1 = DFA::from_string("a", &symbol_table);
        let dfa2 = DFA::from_string("a", &symbol_table);
        let dfa = dfa1.intersection(dfa2);

        let result = dfa.run("a");
        assert!(result.is_ok_and(|res| res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));

        let dfa1 = DFA::from_string("a", &symbol_table);
        let dfa2 = DFA::from_string("", &symbol_table);
        let dfa = dfa1.intersection(dfa2);

        let result = dfa.run("a");
        assert!(result.is_ok_and(|res| !res));

        let result = dfa.run("abd");
        assert!(result.is_ok_and(|res| !res));
    }
}
