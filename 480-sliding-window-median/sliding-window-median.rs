use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct SlidingMedian {
    small: BinaryHeap<i64>,
    large: BinaryHeap<Reverse<i64>>,
    delayed: HashMap<i64, i32>,
    small_size: usize,
    large_size: usize,
}

impl SlidingMedian {
    fn new() -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
            delayed: HashMap::new(),
            small_size: 0,
            large_size: 0,
        }
    }
    fn prune_small(&mut self) {
        while let Some(&x) = self.small.peek() {
            if let Some(&c) = self.delayed.get(&x) {
                if c > 0 {
                    self.delayed.insert(x, c - 1);
                    self.small.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    fn prune_large(&mut self) {
        while let Some(&Reverse(x)) = self.large.peek() {
            if let Some(&c) = self.delayed.get(&x) {
                if c > 0 {
                    self.delayed.insert(x, c - 1);
                    self.large.pop();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    fn balance(&mut self) {
        if self.small_size > self.large_size + 1 {
            if let Some(x) = self.small.pop() {
                self.large.push(Reverse(x));
                self.small_size -= 1;
                self.large_size += 1;
                self.prune_small();
            }
        } else if self.large_size > self.small_size + 1 {
            if let Some(Reverse(x)) = self.large.pop() {
                self.small.push(x);
                self.large_size -= 1;
                self.small_size += 1;
                self.prune_large();
            }
        }
    }
    fn insert(&mut self, num: i64) {
        if self.small.peek().map_or(true, |&t| num <= t) {
            self.small.push(num);
            self.small_size += 1;
        } else {
            self.large.push(Reverse(num));
            self.large_size += 1;
        }
        self.balance();
    }
    fn remove(&mut self, num: i64) {
        *self.delayed.entry(num).or_insert(0) += 1;
        if self.small.peek().map_or(false, |&t| num <= t) {
            self.small_size -= 1;
            if Some(&num) == self.small.peek() {
                self.prune_small();
            }
        } else {
            self.large_size -= 1;
            if Some(&Reverse(num)) == self.large.peek() {
                self.prune_large();
            }
        }
        self.balance();
    }
    fn median(&mut self) -> f64 {
        self.prune_small();
        self.prune_large();
        match (self.small.peek(), self.large.peek()) {
            (Some(&low), Some(&Reverse(high))) => {
                if self.small_size > self.large_size {
                    low as f64
                } else if self.large_size > self.small_size {
                    high as f64
                } else {
                    (low as f64 + high as f64) / 2.0
                }
            }
            (Some(&low), None) => low as f64,
            (None, Some(&Reverse(high))) => high as f64,
            _ => 0.0,
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut med = SlidingMedian::new();
        let n = nums.len();
        let mut ans = Vec::with_capacity(n.saturating_sub(k) + 1);
        for &v in nums.iter().take(k) {
            med.insert(v as i64);
        }
        ans.push(med.median());
        for i in k..n {
            med.insert(nums[i] as i64);
            med.remove(nums[i - k] as i64);
            ans.push(med.median());
        }
        ans
    }
}
