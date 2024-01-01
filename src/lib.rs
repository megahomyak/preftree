use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct PrefixTree<K: Hash + Eq, V> {
    pub value: Option<V>,
    pub subtrees: HashMap<K, PrefixTree<K, V>>,
}

impl<K: Hash + Eq, V> Default for PrefixTree<K, V> {
    fn default() -> Self {
        Self {
            value: None,
            subtrees: HashMap::new(),
        }
    }
}

impl<K: Hash + Eq, V> PrefixTree<K, V> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts the specified value at the specified key; returns the previous value at the same
    /// key if there was one before
    pub fn insert(&mut self, sequence: impl IntoIterator<Item = K>, value: V) -> Option<V> {
        let sequence = sequence.into_iter();
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
    pub fn get_by_shortest_prefix<I: Borrow<K>>(
        &self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<&V> {
        let mut sequence = sequence.into_iter();
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&root.value).into();
            }
            root = match sequence
                .next()
                .and_then(|item| root.subtrees.get(item.borrow()))
            {
                Some(subtree) => subtree,
                None => return None,
            };
        }
    }

    /// Returns a mutable reference to the value associated with the shortest prefix of the given
    /// sequence (or `None` if no prefixes were found)
    pub fn get_by_shortest_prefix_mut<I: Borrow<K>>(
        &mut self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<&mut V> {
        let mut sequence = sequence.into_iter();
        let mut root = self;
        loop {
            if matches!(root.value, Some(_)) {
                return (&mut root.value).into();
            }
            root = match sequence
                .next()
                .and_then(|item| root.subtrees.get_mut(item.borrow()))
            {
                Some(subtree) => subtree,
                None => return None,
            };
        }
    }

    /// Returns a mutable reference to the value associated with the exact match of the given
    /// sequence (or `None` if no such sequence is found)
    pub fn get_exact_match_mut<I: Borrow<K>>(
        &mut self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<&mut V> {
        let sequence = sequence.into_iter();
        let mut root = self;
        for item in sequence {
            root = match root.subtrees.get_mut(item.borrow()) {
                Some(subtree) => subtree,
                None => return None,
            };
        }
        (&mut root.value).into()
    }

    /// Returns an immutable reference to the value associated with the exact match of the
    /// given sequence (or `None` if no such sequence is found)
    pub fn get_exact_match<I: Borrow<K>>(
        &self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<&V> {
        let sequence = sequence.into_iter();
        let mut root = self;
        for item in sequence {
            root = match root.subtrees.get(item.borrow()) {
                Some(subtree) => subtree,
                None => return None,
            };
        }
        (&root.value).into()
    }

    /// Removes the value associated with the exact match of the given sequence from the tree and
    /// returns it (or returns `None` if no matching value was found)
    pub fn remove_exact_match<I: Borrow<K>>(
        &mut self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<V> {
        let sequence = sequence.into_iter();
        let mut root = self;
        let mut keys = Vec::new();
        for item in sequence {
            let old_root = root as *mut _;
            root = match root.subtrees.get_mut(item.borrow()) {
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
            unsafe { (*root).subtrees.remove(item.borrow()) };
            if unsafe { (*root).value.is_some() } {
                break;
            }
        }
        result
    }

    /// Removes the value associated with the shortest prefix of the given sequence from the tree
    /// and returns it (or returns `None` if no matching value was found)
    pub fn remove_by_shortest_prefix<I: Borrow<K>>(
        &mut self,
        sequence: impl IntoIterator<Item = I>,
    ) -> Option<V> {
        let mut sequence = sequence.into_iter();
        let mut root = self;
        let mut keys = Vec::new();
        loop {
            if matches!(root.value, Some(_)) {
                break;
            }
            let old_root = root as *mut _;
            let item;
            (root, item) = match sequence.next().and_then(|item| {
                root.subtrees
                    .get_mut(item.borrow())
                    .map(|subtree| (subtree, item))
            }) {
                Some((subtree, item)) => (subtree, item),
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
            unsafe { (*root).subtrees.remove(item.borrow()) };
            if unsafe { (*root).value.is_some() } {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    macro_rules! tree {
        ($value:expr, $subtrees:expr) => {
            PrefixTree {
                value: $value,
                subtrees: $subtrees,
            }
        };
    }

    #[test]
    fn deletion() {
        let mut tree = PrefixTree::new();

        tree.insert("".chars(), 1);
        tree.insert("a".chars(), 2);
        tree.insert("abc".chars(), 3);

        tree.remove_exact_match("a".chars());

        assert_eq!(
            tree,
            tree!(
                Some(1),
                hashmap! {
                    'a' => tree!(None, hashmap!{
                        'b' => tree!(None, hashmap!{
                            'c' => tree!(Some(3), hashmap!{}),
                        })
                    })
                }
            )
        );

        let mut chars = "abc".chars();

        tree.remove_by_shortest_prefix(&mut chars);

        assert_eq!(
            tree,
            tree!(
                None,
                hashmap! {
                    'a' => tree!(None, hashmap!{
                        'b' => tree!(None, hashmap!{
                            'c' => tree!(Some(3), hashmap!{}),
                        })
                    })
                }
            )
        );

        assert_eq!(chars.as_str(), "abc");
    }
}
