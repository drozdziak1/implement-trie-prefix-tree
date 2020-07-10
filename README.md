# implement-trie-prefix-tree
My take on https://leetcode.com/problems/implement-trie-prefix-tree/ in Rust.

# Design decisions
* I make the structure generic over anything `Default + Eq + Hash`
* I use a HashMap for the children under the assumption that even the
  specialized string case in the original task would need to deal with
  unicode chars in production, which are variable-length as per UTF-*
  spec.
* I add a simple flag to `search()` to achieve `startsWith` behavior and keep things DRY.
