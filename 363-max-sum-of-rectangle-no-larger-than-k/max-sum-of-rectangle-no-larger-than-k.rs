use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut res = i32::MIN;

        for left in 0..cols {
            let mut row_sums = vec![0; rows];

            for right in left..cols {
                for r in 0..rows {
                    row_sums[r] += matrix[r][right];
                }

                let mut set = BTreeSet::new();
                set.insert(0);
                let mut curr_sum = 0;

                for &sum in &row_sums {
                    curr_sum += sum;
                    if let Some(&s) = set.range((curr_sum - k)..).next() {
                        res = res.max(curr_sum - s);
                    }
                    set.insert(curr_sum);
                }
            }
        }

        res
    }
}