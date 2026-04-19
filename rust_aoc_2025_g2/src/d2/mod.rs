#[allow(unused)]
pub fn d2p1_v1(s: &str) -> usize {
    fn is_invalid_id(n: usize) -> bool {
        let text = n.to_string();
        let len = text.len();

        // Must be exactly "some sequence of digits repeated twice"
        if !len.is_multiple_of(2) {
            return false;
        }

        let half = len / 2;
        let (left, right) = text.split_at(half);

        left == right
    }

    let mut password = 0usize;

    for range in s.trim().split(',').filter(|r| !r.is_empty()) {
        let (start, end) = range
            .split_once('-')
            .expect("Each range must be in the form start-end");

        let start: usize = start.parse().expect("Invalid start number");
        let end: usize = end.parse().expect("Invalid end number");

        for id in start..=end {
            if is_invalid_id(id) {
                password += id;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d2p1_v2(s: &str) -> usize {
    // Optimisation: avoid string conversions and allocations by working directly with digits.

    fn is_invalid_id(mut n: usize) -> bool {
        // Count digits
        let mut len = 0;
        let mut tmp = n;
        while tmp > 0 {
            len += 1;
            tmp /= 10;
        }

        // 0 would not be a valid ID in this puzzle context, but handle it safely
        if len == 0 || len % 2 != 0 {
            return false;
        }

        let half = len / 2;
        let pow10 = 10usize.pow(u32::try_from(half).unwrap());

        let right = n % pow10;
        let left = n / pow10;

        left == right
    }

    let mut password = 0usize;

    for range in s.trim().split(',').filter(|r| !r.is_empty()) {
        let (start, end) = range
            .split_once('-')
            .expect("Each range must be in the form start-end");

        let start: usize = start.parse().expect("Invalid start number");
        let end: usize = end.parse().expect("Invalid end number");

        for id in start..=end {
            if is_invalid_id(id) {
                password += id;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d2p2_v1(s: &str) -> usize {
    fn is_invalid_id(n: usize) -> bool {
        let text = n.to_string();
        let len = text.len();

        // Try every possible pattern length that could repeat
        // to make the full string, with at least 2 repetitions.
        for pattern_len in 1..=len / 2 {
            if !len.is_multiple_of(pattern_len) {
                continue;
            }

            let repeat_count = len / pattern_len;
            if repeat_count < 2 {
                continue;
            }

            let pattern = &text[..pattern_len];
            let rebuilt = pattern.repeat(repeat_count);

            if rebuilt == text {
                return true;
            }
        }

        false
    }

    let mut password = 0usize;

    for range in s.trim().split(',').filter(|r| !r.is_empty()) {
        let (start, end) = range
            .split_once('-')
            .expect("Each range must be in the form start-end");

        let start: usize = start.parse().expect("Invalid start number");
        let end: usize = end.parse().expect("Invalid end number");

        for id in start..=end {
            if is_invalid_id(id) {
                password += id;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d2p2_v2(s: &str) -> usize {
    // Optimisation: avoid string conversions and allocations by working directly with digits.
    
    fn is_invalid_id(n: usize) -> bool {
        // Count digits
        let mut len = 0;
        let mut tmp = n;
        while tmp > 0 {
            len += 1;
            tmp /= 10;
        }

        // 0 is not a meaningful ID here, and we need at least 2 repetitions
        if len < 2 {
            return false;
        }

        // Precompute powers of 10 up to len
        let mut pow10 = vec![1usize; len + 1];
        for i in 1..=len {
            pow10[i] = pow10[i - 1] * 10;
        }

        // Try every possible block length that divides the total length
        for (block_len, &pow10_val) in pow10.iter().enumerate().take(len / 2 + 1).skip(1) {
            if !len.is_multiple_of(block_len) {
                continue;
            }

            let repeats = len / block_len;
            if repeats < 2 {
                continue;
            }

            let base = pow10[block_len];
            let pattern = n % base;

            let mut x = n;
            let mut valid = true;

            for _ in 0..repeats {
                if x % base != pattern {
                    valid = false;
                    break;
                }
                x /= base;
            }

            if valid {
                return true;
            }
        }

        false
    }

    let mut password = 0usize;

    for range in s.trim().split(',').filter(|r| !r.is_empty()) {
        let (start, end) = range
            .split_once('-')
            .expect("Each range must be in the form start-end");

        let start: usize = start.parse().expect("Invalid start number");
        let end: usize = end.parse().expect("Invalid end number");

        for id in start..=end {
            if is_invalid_id(id) {
                password += id;
            }
        }
    }

    password
}

#[allow(unused)]
pub fn d2p1(s: &str) -> usize {
    d2p1_v2(s)
}
#[allow(unused)]
pub fn d2p2(s: &str) -> usize {
    d2p2_v2(s)
}

#[cfg(test)]
mod tests {
    use crate::d2::{d2p1, d2p2};

    #[test]
    fn d2p1_test() {
        let s = include_str!("d2_test.txt");
        let result: usize = d2p1(s);
        println!("result: {}", result);
        assert_eq!(1_227_775_554, result);
    }

    #[test]
    fn d2p2_test() {
        let s = include_str!("d2_test.txt");
        let result: usize = d2p2(s);
        println!("result: {}", result);
        assert_eq!(4_174_379_265, result);
    }
}
