use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let word_set: HashSet<String> = words.iter().cloned().collect();
        let mut memo: HashMap<String, bool> = HashMap::new();
        let mut result = Vec::new();

        for word in &words {
            if Self::can_form(word, &word_set, &mut memo) {
                result.push(word.clone());
            }
        }

        result
    }

    fn can_form(word: &str, word_set: &HashSet<String>, memo: &mut HashMap<String, bool>) -> bool {
        if let Some(&cached) = memo.get(word) {
            return cached;
        }

        for i in 1..word.len() {
            let prefix = &word[..i];
            let suffix = &word[i..];

            if word_set.contains(prefix) {
                if word_set.contains(suffix) || Self::can_form(suffix, word_set, memo) {
                    memo.insert(word.to_string(), true);
                    return true;
                }
            }
        }

        memo.insert(word.to_string(), false);
        false
    }
}