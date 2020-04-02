#![feature(test)]

use anagram;
extern crate test;

#[bench]
fn case_insensitive_anagrams(b: &mut test::Bencher) {
    let word = "Orchestra";
    let inputs = ["cashregister", "Carthorse", "radishes"];
    b.iter(|| {
        anagram::anagrams_for(word, &inputs);
    });
}
