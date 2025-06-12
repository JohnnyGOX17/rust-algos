//! #[3442. Maximum Difference Between Even and Odd Frequency I (Easy)](https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/description/)
//!
//! You are given a string `s` consisting of lowercase English letters.
//! Your task is to find the maximum difference `diff = freq(a1) - freq(a2)` between the frequency of characters `a1` and `a2` in the string such that:
//! - `a1` has an odd frequency in the string.
//! - `a2` has an even frequency in the string.
//!
//! Return this maximum difference.

pub fn max_difference(s: String) -> i32 {
    let mut char_counts = [0i32; 26];
    for c in s.chars() {
        let idx = c as usize - 97;
        char_counts[idx] += 1;
    }

    let mut max_odd = 1;
    let mut min_even = i32::MAX;

    for count in char_counts {
        if count % 2 == 1 {
            if count > max_odd {
                max_odd = count;
            }
        } else if count < min_even && count > 1 {
            min_even = count;
        }
    }
    max_odd - min_even
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s = "aaaaabbc".to_string();
        assert_eq!(max_difference(s), 3);
    }

    #[test]
    fn case_2() {
        let s = "abcabcab".to_string();
        assert_eq!(max_difference(s), 1);
    }

    #[test]
    fn case_3() {
        let s = "szsseeeusuu".to_string();
        assert_eq!(max_difference(s), -1);
    }
}
