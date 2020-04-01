use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let mut word_freq = HashMap::new();
    for c in word_lower.chars() {
        *word_freq.entry(c.escape_unicode().to_string()).or_insert(0) += 1;
    }

    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for possible in possible_anagrams {
        // if words have different length they can't be anagrams
        if possible.chars().count() != word.chars().count() {
            continue;
        }
        let mut possible_freq = word_freq.clone();
        let possible_lower = possible.to_lowercase();
        for c in possible_lower.chars() {
            if !possible_freq.contains_key(&c.escape_unicode().to_string()) {
                break;
            }
            *possible_freq
                .entry(c.escape_unicode().to_string())
                .or_insert(1) -= 1;
        }

        possible_freq.retain(|_, v| *v != 0);
        if possible_freq.is_empty() && word_lower != possible_lower {
            anagrams.insert(possible);
        }
    }
    anagrams
}
