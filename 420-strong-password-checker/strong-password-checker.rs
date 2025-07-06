impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let n = password.len();
        let bytes = password.as_bytes();
        let (mut has_lower, mut has_upper, mut has_digit) = (false, false, false);
        let mut rep = vec![];
        let mut i = 0;

        while i < n {
            let c = bytes[i];
            if c.is_ascii_lowercase() { has_lower = true; }
            if c.is_ascii_uppercase() { has_upper = true; }
            if c.is_ascii_digit() { has_digit = true; }

            let mut j = i;
            while j < n && bytes[j] == c { j += 1; }
            if j - i >= 3 {
                rep.push(j - i);
            }
            i = j;
        }

        let missing = (!has_lower as i32) + (!has_upper as i32) + (!has_digit as i32);

        if n < 6 {
            return (6 - n as i32).max(missing);
        }

        let mut over = n as i32 - 20;
        let mut to_replace = 0;
        let mut mods = vec![0; 3];
        for r in rep {
            to_replace += (r / 3) as i32;
            mods[r % 3] += 1;
        }

        if over > 0 {
            let mut del = over;
            for i in 0..3 {
                let mut m = mods[i];
                let mut reduce = i as i32 + 1;
                while m > 0 && del >= reduce {
                    del -= reduce;
                    to_replace -= 1;
                    m -= 1;
                }
            }
            to_replace -= del / 3;
            over + missing.max(to_replace)
        } else {
            missing.max(to_replace)
        }
    }
}