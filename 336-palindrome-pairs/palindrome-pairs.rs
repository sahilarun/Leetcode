use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        fn is_palindrome(s: &str) -> bool {
            let bytes = s.as_bytes();
            let (mut l, mut r) = (0, bytes.len().saturating_sub(1));
            while l < r {
                if bytes[l] != bytes[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        let mut word_map: HashMap<String, usize> = HashMap::new();
        for (i, word) in words.iter().enumerate() {
            word_map.insert(word.chars().rev().collect::<String>(), i);
        }

        let mut result = Vec::new();

        for (i, word) in words.iter().enumerate() {
            let len = word.len();
            for j in 0..=len {
                let (left, right) = word.split_at(j);

                if is_palindrome(left) {
                    if let Some(&idx) = word_map.get(right) {
                        if idx != i {
                            result.push(vec![idx as i32, i as i32]);
                        }
                    }
                }

                if j < len && is_palindrome(right) {
                    if let Some(&idx) = word_map.get(left) {
                        if idx != i {
                            result.push(vec![i as i32, idx as i32]);
                        }
                    }
                }
            }
        }

        result
    }
}
