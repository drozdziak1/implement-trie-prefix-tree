use std::{collections::HashMap, hash::Hash};

#[derive(Default)]
struct Trie<T> {
    children: HashMap<T, Trie<T>>,
    is_end: bool,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
