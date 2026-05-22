// NOTE: WE'RE NOT TAKING THIS APPROACH ANY MORE!

use std::{collections::HashMap, u32};
enum Inline {
    Text(String),
    Ref {
        text: String,
        target: u32,
    },
}
struct Trie {
    root: TrieNode
}
struct TrieNode {
    children: HashMap<String, TrieNode>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode {
                children: HashMap::new(),
                is_end: false,
            }
        }
    }

    fn insert(&mut self, term: &str) {
        let mut node = &mut self.root;

        for word in term.split_whitespace() {
            node = node.children
                .entry(word.to_lowercase()) // get matching child if it exists
                .or_insert( // and if the entry is empty (has no associated value), inserts the passed default value 
                    TrieNode {
                        children: HashMap::new(),
                        is_end: false
                    }
                );
        }
        node.is_end = true;
    }

    fn find_matches (&mut self, search_term: &str) { // need to implement taking into account plurality
        
        let mut root = &mut self.root;
        let mut end = false;
    }

}

impl TrieNode {

}