use std::{collections::HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let lower_word: String = word.to_lowercase();
    let mut word_marker: [usize; 100_000] = [0; 100_000];

    for letter in lower_word.chars() {
        let current = letter as usize;
        word_marker[current] += 1;
    }

    for possible in possible_anagrams {
        let mut anag_marker: [usize; 100_000] = [0; 100_000];
        let lower_anagram: String = possible.to_lowercase();

        if lower_word == lower_anagram {
            continue;
        }

        for letter in lower_anagram.chars() {
            let current = letter as usize;
            anag_marker[current] += 1;
        }

        if anag_marker == word_marker {
            anagrams.insert(possible);
        }
    }

    anagrams
}
