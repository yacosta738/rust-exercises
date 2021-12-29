use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    // using unicode utf-8 and lowercase
    let word_lower = word.to_lowercase();
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort();
    let word_chars_str: String = word_chars.into_iter().collect();
    for possible_anagram in possible_anagrams {
        let possible_anagram_lower = possible_anagram.to_lowercase();
        let mut possible_anagram_chars: Vec<char> = possible_anagram_lower.chars().collect();
        possible_anagram_chars.sort();
        let possible_anagram_chars_str: String = possible_anagram_chars.into_iter().collect();
        if word_chars_str == possible_anagram_chars_str && word_lower != possible_anagram_lower {
            result.insert(possible_anagram);
        }
    }
    result
}
