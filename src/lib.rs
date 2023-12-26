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

    /// Inserts the specified value at the specified key; returns the previous value at the same
    /// key, if there was one
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

    /// Returns an immutable reference to the value associated with the shortest prefix of the
    /// given sequence (or `None` if no prefixes were found)
    pub fn get_by_shortest_prefix<I: AsRef<K>>(
        &self,
        mut sequence: impl Iterator<Item = I>,
    ) -> Option<&V> {
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&root.value).into();
            }
            root = match sequence
                .next()
                .and_then(|item| root.subtrees.get(item.as_ref()))
            {
                Some(subtree) => subtree,
                None => return None,
            };
        }
    }

    /// Returns a mutable reference to the value associated with the shortest prefix of the given
    /// sequence (or `None` if no prefixes were found)
    pub fn get_by_shortest_prefix_mut<I: AsRef<K>>(
        &mut self,
        mut sequence: impl Iterator<Item = I>,
    ) -> Option<&mut V> {
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&mut root.value).into();
            }
            root = match sequence
                .next()
                .and_then(|item| root.subtrees.get_mut(item.as_ref()))
            {
                Some(subtree) => subtree,
                None => return None,
            };
        }
    }

    /// Returns a mutable reference to the value associated with the exact match of the given
    /// sequence (or `None` if no such sequence is found)
    pub fn get_exact_match_mut<I: AsRef<K>>(
        &mut self,
        sequence: impl Iterator<Item = I>,
    ) -> Option<&mut V> {
        let mut root = self;
        for item in sequence {
            root = match root.subtrees.get_mut(item.as_ref()) {
                Some(subtree) => subtree,
                None => return None,
            }
        }
        (&mut root.value).into()
    }

    /// Returns an immutable mutable reference to the value associated with the exact match of the
    /// given sequence (or `None` if no such sequence is found)
    pub fn get_exact_match<I: AsRef<K>>(&self, sequence: impl Iterator<Item = I>) -> Option<&V> {
        let mut root = self;
        for item in sequence {
            root = match root.subtrees.get(item.as_ref()) {
                Some(subtree) => subtree,
                None => return None,
            }
        }
        (&root.value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
