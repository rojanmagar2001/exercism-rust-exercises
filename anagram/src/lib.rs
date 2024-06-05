use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let sorted_word = sort_characters(word);

    let anagrams = possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let sorted_possible_anagram = sort_characters(possible_anagram);
            sorted_word == sorted_possible_anagram
                && word.to_lowercase() != possible_anagram.to_lowercase()
        })
        .copied()
        .collect::<HashSet<&str>>();

    anagrams
}

fn sort_characters(word: &str) -> String {
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect()
}
