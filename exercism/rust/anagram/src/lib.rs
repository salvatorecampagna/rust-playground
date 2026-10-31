use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let lowercase_word = word.to_lowercase();
    let lowercase_word_vec = sorted_chars(&lowercase_word);
    for &possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        if lowercase_word == lowercase_possible_anagram {
            continue;
        }
        let lowercase_possible_anagram_vec = sorted_chars(&lowercase_possible_anagram);
        if lowercase_word_vec == lowercase_possible_anagram_vec {
            result.insert(possible_anagram);
        }
    }
    result
}

fn sorted_chars(s: &str) -> Vec<char> {
    let mut chars = s.trim().chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars
}
