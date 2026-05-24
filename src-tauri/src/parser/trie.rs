use std::{char, collections::{HashSet, VecDeque}, i32};
use std::collections::BTreeSet;
use crate::utils::matrix::Matrix;

pub struct AhoCorasick {
    max_s: i32, // sum of lengths of all terms
    max_c: i32, // number of characters in input alphabet
    
    // OUTPUT FUNCTION:
    // Bit i in this mask is one if the word with index i appears when the machine enters this state
    out: Vec<i32>, // out[i] = 'is ith node an accept state?'
    failure: Vec<i32>, // failure[i] = index of node to fallback to`
                       // eg. if string 'ab' at index i and next character 'c', and we have term 'bc' but not term 'abc', failure[i] would be index of string 'b', then from there reading 'c' will transition to state 'bc'
    trie: Matrix<i32>, // for each node i, trie[i] contains a list of indices of all adjacent nodes
                       // eg. if we are currently at node 'ab' (suppose it is at index i), and we have terms 'abc' and 'abed', then we will have trie[i]['c'] --> (index of node for 'abc'), and trie[i]['e'] --> (index of node for 'abe')
}


// need a function that takes an array of strings (terms) that are defined, and from them get a
// list of the unique characters these strings use, sort them, enumerate them, and return a map of
// char <--> int encoding
// we need this since we are not just constraining our potentially defined terms to just the
// lowercase alphabet (26 values), we want them to be able to include numbers, special characters
// ($, %, -, etc.)
fn build_charmap(terms: Vec<String>) -> Vec<char> {
    let mut chars = BTreeSet::new();

    for term in terms {
        for c in term.chars() {
            chars.insert(c);
        }
    }

    chars.into_iter().collect()
}


