impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }

        let upper = 10i64.pow(n as u32) - 1;
        let lower = 10i64.pow(n as u32 - 1);

        for a in (lower..=upper).rev() {
            let pal = Self::make_palindrome(a);
            for b in (lower..=upper).rev() {
                if pal / b > upper {
                    break;
                }
                if pal % b == 0 {
                    return (pal % 1337) as i32;
                }
            }
        }

        0
    }

    fn make_palindrome(x: i64) -> i64 {
        let s = x.to_string();
        let rev: String = s.chars().rev().collect();
        let pal = format!("{}{}", s, rev);
        pal.parse::<i64>().unwrap()
    }
}