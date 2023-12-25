use std::collections::HashMap;
use std::hash::Hash;

pub struct PrefixTree<K, V> {
    value: Option<V>,
    subtrees: HashMap<K, PrefixTree<K, V>>,
}

impl<K: Hash + Eq, V> PrefixTree<K, V> {
    pub fn new() -> Self {
        Self {
            value: None,
            subtrees: HashMap::new(),
        }
    }

    /// Returns the previous value at the same key, if there was one
    pub fn insert(&mut self, sequence: impl Iterator<Item = K>, value: V) -> Option<V> {
        let mut root = self;
        for item in sequence {
            root = root
                .subtrees
                .entry(item)
                .or_insert_with(|| PrefixTree::new());
        }
        root.value.replace(value)
    }

    /// Returns a reference to the value associated with the shortest prefix of the given sequence
    /// (or `None` if no prefixes were found)
    pub fn get_by_shortest_prefix<'a>(&self, mut sequence: impl Iterator<Item = &'a K>) -> Option<&V> {
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&root.value).into();
            }
            let Some(new_root) = sequence.next().and_then(|item| root.subtrees.get(item)) else {
                return None;
            };
            root = new_root;
        }
    }

    /// Returns a mutable reference to the value associated with the shortest prefix of the given
    /// sequence (or `None` if no prefixes were found)
    pub fn get_by_shortest_prefix_mut<'a>(&mut self, mut sequence: impl Iterator<Item = &'a K> + 'a) -> Option<&mut V> {
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&mut root.value).into();
            }
            let Some(new_root) = sequence.next().and_then(|item| root.subtrees.get_mut(item)) else {
                return None;
            };
            root = new_root;
        }
    }

    /// Returns a mutable reference to the value associated with the exact match of the given
    /// sequence (or `None` if no such sequence is found)
    pub fn get_exact_match_mut<'a>(&mut self, mut sequence: impl Iterator<Item = &'a K>) -> Option<&mut V> {
        let mut root = self;
        for item in sequence {
            let Some(new_root) = root.subtrees.get_mut(item) {}
        }
        &mut root.value
        loop {
            if matches!(root.value, Some(_)) {
                return (&mut root.value).into();
            }
            let Some(new_root) = sequence.next().and_then(|item| root.subtrees.get_mut(&item)) else {
                return None;
            };
            root = new_root;
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
