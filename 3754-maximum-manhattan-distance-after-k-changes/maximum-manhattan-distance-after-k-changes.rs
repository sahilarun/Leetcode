impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let (mut x, mut y): (i32, i32) = (0, 0);
        let mut max_dist = 0;

        for (i, ch) in s.bytes().enumerate() {
            match ch {
                b'N' => y += 1,
                b'S' => y -= 1,
                b'E' => x += 1,
                b'W' => x -= 1,
                _ => {}
            }
            let steps = (i + 1) as i32;
            let dist = x.abs() + y.abs();
            let extra = 2 * k.min(steps);
            max_dist = max_dist.max((dist + extra).min(steps));
        }

        max_dist
    }
}