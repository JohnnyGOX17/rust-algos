//! #[1790. Check if One String Swap Can Make Strings Equal (Easy)](https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/description/)
//!
//! You are given two strings s1 and s2 of equal length. A string swap is an operation where you
//! choose two indices in a string (not necessarily different) and swap the characters at these
//! indices. Return true if it is possible to make both strings equal by performing at most one
//! string swap on exactly one of the strings. Otherwise, return false.
//!
//! Constraints:
//! * 1 <= s1.length, s2.length <= 100
//! * s1.length == s2.length
//! * s1 and s2 consist of only lowercase English letters.

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1 == s2 {
        return true;
    }

    let mut s1_char = 'a';
    let mut s2_char = 'a';
    let mut diff_seen = false;
    let mut more_than_two_diff = false;

    for (ch1, ch2) in s1.chars().zip(s2.chars()) {
        if ch1 != ch2 {
            if more_than_two_diff {
                return false;
            }

            if !diff_seen {
                s1_char = ch1;
                s2_char = ch2;
                diff_seen = true;
            } else if (s1_char != ch2) || (s2_char != ch1) {
                return false;
            } else {
                more_than_two_diff = true;
            }
        }
    }

    !diff_seen || more_than_two_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s1 = "bank".to_string();
        let s2 = "kanb".to_string();
        assert!(are_almost_equal(s1, s2));
    }

    #[test]
    fn case_2() {
        let s1 = "attack".to_string();
        let s2 = "defend".to_string();
        assert!(!are_almost_equal(s1, s2));
    }

    #[test]
    fn case_3() {
        let s1 = "kelb".to_string();
        let s2 = "kelb".to_string();
        assert!(are_almost_equal(s1, s2));
    }

    #[test]
    fn case_4() {
        let s1 = "aa".to_string();
        let s2 = "ac".to_string();
        assert!(!are_almost_equal(s1, s2));
    }
}
