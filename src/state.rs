use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

pub type State = usize;

#[derive(Clone, Debug)]
pub struct StateSet(HashSet<State>);

impl StateSet {
    pub fn new(set: HashSet<State>) -> Self {
        StateSet(set)
    }

    pub fn states(&self) -> &HashSet<State> {
        &self.0
    }
}

impl PartialEq for StateSet {
    fn eq(&self, other: &StateSet) -> bool {
        self.0 == other.0
    }
}

impl Eq for StateSet {}

impl Hash for StateSet {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        let mut a: Vec<&State> = self.0.iter().collect();
        a.sort();
        for s in a.iter() {
            s.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_hash_of_empty() {
        let empty = HashSet::new();

        let empty1 = StateSet::new(empty.clone());
        let empty2 = StateSet::new(empty);
        assert_eq!(empty1, empty2);
    }
}
