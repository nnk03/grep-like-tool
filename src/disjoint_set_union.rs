#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use crate::state::State;

pub struct DSU {
    parent: Vec<State>,
}

impl DSU {
    /// number of elements to initialise DSU with
    pub fn new(n: usize) -> DSU {
        let mut dsu = DSU { parent: vec![0; n] };
        for i in 0..n {
            dsu.parent[i] = i;
        }

        dsu
    }

    /// function to return the number of disjoint sets
    pub fn len(&self) -> usize {
        let mut distinct_set_representatives = HashSet::new();
        for &state in self.parent.iter() {
            distinct_set_representatives.insert(state);
        }

        distinct_set_representatives.len()
    }

    /// function to return a map of state to its parent
    pub fn state_representative_map(&self, offset: usize) -> HashMap<State, State> {
        let mut map = HashMap::new();

        for (state, &representative) in self.parent.iter().enumerate() {
            map.insert(state + offset, representative + offset);
        }

        map
    }

    /// function to find the representative of the current set
    pub fn find_representative(&mut self, state: State) -> State {
        if self.parent[state] == state {
            return state;
        }

        let par = self.find_representative(self.parent[state]);
        self.parent[state] = par;

        return par;
    }

    /// function to unite 2 disjoint sets
    pub fn union(&mut self, u: State, v: State) {
        let u_p = self.find_representative(u);
        let v_p = self.find_representative(v);

        if u_p == v_p {
            return;
        }

        // make the smaller one, the parent of the larger one
        if u_p < v_p {
            self.parent[v_p] = u_p;
        } else {
            self.parent[u_p] = v_p;
        }
    }
}
