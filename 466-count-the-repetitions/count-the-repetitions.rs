use std::collections::HashMap;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        if n1 == 0 {
            return 0;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();

        let mut indexr = 0;
        let mut count1 = 0;
        let mut count2 = 0;
        let mut recall: HashMap<usize, (i32, i32)> = HashMap::new();

        while count1 < n1 {
            for &c in &s1_chars {
                if c == s2_chars[indexr] {
                    indexr += 1;
                    if indexr == s2_chars.len() {
                        indexr = 0;
                        count2 += 1;
                    }
                }
            }
            count1 += 1;

            if let Some(&(prev_count1, prev_count2)) = recall.get(&indexr) {
                // cycle found
                let cycle_len1 = count1 - prev_count1;
                let cycle_len2 = count2 - prev_count2;
                let remaining = (n1 - count1) / cycle_len1;

                count1 += remaining * cycle_len1;
                count2 += remaining * cycle_len2;
            } else {
                recall.insert(indexr, (count1, count2));
            }
        }

        count2 / n2
    }
}