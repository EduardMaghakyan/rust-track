use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut word_letters = HashMap::new();
    let word = word.to_lowercase();
    for c in word.chars() {
        *word_letters
            .entry(c.escape_unicode().to_string())
            .or_insert(1) += 1;
    }

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    // let mut anagrams = HashSet::new();
    for possible in possible_anagrams {
        let mut possible_letters = HashMap::new();
        for c in possible.to_lowercase().chars() {
            *possible_letters
                .entry(c.escape_unicode().to_string())
                .or_insert(1) += 1;
        }

        if word_letters == possible_letters && word != possible.to_lowercase() {
            anagrams.insert(possible);
        }
    }
    anagrams
}
