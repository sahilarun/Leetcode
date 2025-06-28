impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
        let mut dp = Vec::new();
        for e in envelopes {
            let h = e[1];
            match dp.binary_search(&h) {
                Ok(i) | Err(i) => {
                    if i == dp.len() { dp.push(h); } else { dp[i] = h; }
                }
            }
        }
        dp.len() as i32
    }
}