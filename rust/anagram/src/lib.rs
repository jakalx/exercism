use hashbag::HashBag;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let chars: HashBag<char> = word.to_lowercase().chars().collect();
    possible_anagrams
        .iter()
        .map(|w| *w)
        .filter(|w| w.to_lowercase() != word.to_lowercase())
        .filter(|w| w.to_lowercase().chars().collect::<HashBag<char>>() == chars)
        .collect()
}
