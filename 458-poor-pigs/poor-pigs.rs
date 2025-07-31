impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let t = minutes_to_test / minutes_to_die;
        let mut pigs: u32 = 0;
        while (t + 1).pow(pigs) < buckets {
            pigs += 1;
        }
        pigs as i32
    }
}