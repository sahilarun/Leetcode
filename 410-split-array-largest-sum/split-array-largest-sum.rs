impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (*nums.iter().max().unwrap(), nums.iter().sum());
        let mut ans = right;

        while left <= right {
            let mid = left + (right - left) / 2;
            if Solution::can_split(&nums, k, mid) {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ans
    }

    fn can_split(nums: &[i32], k: i32, max_sum: i32) -> bool {
        let mut count = 1;
        let mut current_sum = 0;

        for &num in nums {
            if current_sum + num > max_sum {
                count += 1;
                current_sum = num;
            } else {
                current_sum += num;
            }
        }

        count <= k
    }
}