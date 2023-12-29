﻿pub mod anagram {
    use std::collections::HashSet;
 
    pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
        let word = word.to_lowercase();
        let mut anagrams : HashSet<&str> = HashSet::new();


        for w in possible_anagrams.into_iter() {
            let candidate = w.to_lowercase();

            if candidate.len() != word.len() || candidate.to_string() == word {
                continue;
            }
            let mut sorted_w = word.chars().collect::<Vec<char>>();
            let mut sorted_candidate = candidate.chars().collect::<Vec<char>>();   

            sorted_w.sort();
            sorted_candidate.sort();

            if sorted_w.eq(&sorted_candidate) {
               
                anagrams.insert(w);
            }
        }
        anagrams
    }
}

use std::collections::HashSet;

fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagram::anagrams_for(word, inputs);
        let expected: HashSet<&str> = expected.iter().cloned().collect();
        assert_eq!(result, expected);
    }
#[test]
fn no_matches() {
    let word = "diaper";
    let inputs = ["hello", "world", "zombies", "pants"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn detect_simple_anagram() {
    let word = "ant";
    let inputs = ["tan", "stand", "at"];
    let outputs = vec!["tan"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn does_not_confuse_different_duplicates() {
    let word = "galea";
    let inputs = ["eagle"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn eliminate_anagram_subsets() {
    let word = "good";
    let inputs = ["dog", "goody"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn detect_anagram() {
    let word = "listen";
    let inputs = ["enlists", "google", "inlets", "banana"];
    let outputs = vec!["inlets"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn multiple_anagrams() {
    let word = "allergy";
    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    let outputs = vec!["gallery", "regally", "largely"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn case_insensitive_anagrams() {
    let word = "Orchestra";
    let inputs = ["cashregister", "Carthorse", "radishes"];
    let outputs = vec!["Carthorse"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn unicode_anagrams() {
    let word = "ΑΒΓ";
    // These words don't make sense, they're just greek letters cobbled together.
    let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
    let outputs = vec!["ΒΓΑ", "γβα"];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn misleading_unicode_anagrams() {
    // Despite what a human might think these words contain different letters, the input uses Greek
    // A and B while the list of potential anagrams uses Latin A and B.
    let word = "ΑΒΓ";
    let inputs = ["ABΓ"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn does_not_detect_a_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["banana"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn does_not_detect_a_differently_cased_word_as_its_own_anagram() {
    let word = "banana";
    let inputs = ["bAnana"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
    let word = "ΑΒΓ";
    let inputs = ["ΑΒγ"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn same_bytes_different_chars() {
    let word = "a⬂"; // 61 E2 AC 82
    let inputs = ["€a"]; // E2 82 AC 61
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}
#[test]
fn different_words_but_same_ascii_sum() {
    let word = "bc";
    let inputs = ["ad"];
    let outputs = vec![];
    process_anagram_case(word, &inputs, &outputs);
}