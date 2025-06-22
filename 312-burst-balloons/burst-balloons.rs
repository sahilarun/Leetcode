impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let mut nums = {
            let mut v = vec![1];
            v.extend(nums.into_iter().filter(|&x| x > 0));
            v.push(1);
            v
        };
        
        let n = nums.len();
        let mut dp = vec![vec![0; n]; n];

        for len in 2..n {
            for left in 0..n - len {
                let right = left + len;
                for i in left + 1..right {
                    dp[left][right] = dp[left][right].max(
                        nums[left] * nums[i] * nums[right] + dp[left][i] + dp[i][right]
                    );
                }
            }
        }

        dp[0][n - 1]
    }
}
