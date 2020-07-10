use std::{collections::HashMap, hash::Hash, iter::IntoIterator};

/// Prefix tree implementation.
#[derive(Default)]
pub struct Trie<T> {
    /// Possible next sequence elements and their respective subtrees
    children: HashMap<T, Trie<T>>,
    /// True if this node is the end of one of the inserted sequences
    is_end: bool,
}

impl<T: Default + Eq + Hash> Trie<T> {
    /// Construct a Trie<T> from an iterable collection of Ts.
    pub fn from_sequence(sequence: impl IntoIterator<Item = T>) -> Self {
        let mut t: Self = Default::default();
        t.insert(sequence);
        return t;
    }

    /// Add a sequence of Ts into the trie
    pub fn insert(&mut self, sequence: impl IntoIterator<Item = T>) {
        let mut last_node = sequence.into_iter().fold(self, |current_node, elem| {
            current_node
                .children
                .entry(elem)
                .or_insert(Default::default())
        });
        last_node.is_end = true;
    }

    /// Look up whether `sequence` is in the trie. `must_end` set to
    /// `true` treats `sequence` as a prefix, i.e. ignores `is_end`
    pub fn search(&self, sequence: impl IntoIterator<Item = T>, is_prefix: bool) -> bool {
        let mut current_node = self;

        for elem in sequence.into_iter() {
            if let Some(next) = current_node.children.get(&elem) {
                current_node = next;
            } else {
                return false;
            }
        }

        current_node.is_end || is_prefix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial_trie_finds_nothing() {
        let t: Trie<char> = Default::default();

        assert_eq!(t.search("Hello".chars(), false), false);
        assert_eq!(t.search("Hello".chars(), true), false);
    }

    #[test]
    fn test_empty_seq_can_be_member() {
	let t: Trie<char> = Trie::from_sequence("".chars());

	assert_eq!(t.search("".chars(), false), true);
	assert_eq!(t.search("".chars(), true), true);
    }

    #[test]
    fn test_trivial_search_succeeds() {
        let txt = "Hello";

        let t = Trie::from_sequence(txt.chars());

        assert_eq!(t.search(txt.chars(), true), true);
        assert_eq!(t.search(txt.chars(), false), true);
    }

    #[test]
    fn test_prefix_found_only_when_intended() {
        let txt = "Hello, World!";

        let mut t = Trie::from_sequence(txt.chars());
        let txt_subseq = "Hello";

	// Prefix not added yet, found if true is passed
        assert_eq!(t.search(txt_subseq.chars(), false), false);
        assert_eq!(t.search(txt_subseq.chars(), true), true);

        t.insert(txt_subseq.chars());

        assert_eq!(t.search(txt_subseq.chars(), false), true);
        assert_eq!(t.search(txt_subseq.chars(), true), true);
    }
}
