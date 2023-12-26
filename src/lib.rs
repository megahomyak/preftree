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
    /// key if there was one before
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
            };
        }
        (&mut root.value).into()
    }

    /// Returns an immutable reference to the value associated with the exact match of the
    /// given sequence (or `None` if no such sequence is found)
    pub fn get_exact_match<I: AsRef<K>>(&self, sequence: impl Iterator<Item = I>) -> Option<&V> {
        let mut root = self;
        for item in sequence {
            root = match root.subtrees.get(item.as_ref()) {
                Some(subtree) => subtree,
                None => return None,
            };
        }
        (&root.value).into()
    }

    /// Removes empty subtrees from the tree to decrease memory consumption
    ///
    /// Meaning of arguments:
    /// * `last_root` - the farthest root from the base root. Should have its `value` set to
    ///   `None`, otherwise there will be information loss
    /// * `roots` - all the other roots, from the base one to the one that is before the
    ///   `last_root`, with keys that were used with them to retrieve the next root
    fn remove_empty_subtrees<I: AsRef<K>>(&self, last_root: &mut Self, roots: Vec<(*mut Self, I)>) {

    }

    /// Removes the value associated with the exact match of the given sequence from the tree and
    /// returns it (or returns `None` if no matching value was found)
    pub fn remove_exact_match<I: AsRef<K>>(
        &mut self,
        sequence: impl Iterator<Item = I>,
    ) -> Option<V> {
        let mut root = self;
        let mut keys = Vec::new();
        for item in sequence {
            let old_root = root as *mut _;
            root = match root.subtrees.get_mut(item.as_ref()) {
                Some(subtree) => subtree,
                None => return None,
            };
            keys.push((old_root, item));
        }
        let result = root.value.take();
        let mut roots = keys.into_iter().rev();
        let mut root: *mut _ = root;
        loop {
            if !unsafe { (*root).subtrees.is_empty() } {
                break;
            }
            let item;
            (root, item) = match roots.next() {
                Some((root, item)) => (root, item),
                None => break,
            };
            unsafe { (*root).subtrees.remove(item.as_ref()) };
            if unsafe { (*root).value.is_some() } {
                break;
            }
        }
        result
    }

    /// Removes the value associated with the shortest prefix of the given sequence from the tree
    /// and returns it (or returns `None` if no matching value was found)
    pub fn remove_by_shortest_prefix<I: AsRef<K>>(
        &mut self,
        sequence: impl Iterator<Item = I>,
    ) -> Option<V> {
        let mut root = self;
        let mut keys = Vec::new();
        loop {
            if root
        }
        for item in sequence {
            let key = root as *mut _;
            root = match root.subtrees.get_mut(item.as_ref()) {
                Some(subtree) => subtree,
                None => return None,
            };
            keys.push((key, item));
        }
        let result = root.value.take();
        let mut keys = keys.into_iter().rev();
        let mut root: *mut _ = root;
        loop {
            if !unsafe { (*root).subtrees.is_empty() } {
                break;
            }
            let key;
            (root, key) = match keys.next() {
                Some((root, key)) => (root, key),
                None => break,
            };
            unsafe { (*root).subtrees.remove(key.as_ref()) };
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
