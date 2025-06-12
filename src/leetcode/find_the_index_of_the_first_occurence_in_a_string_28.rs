//! #[28. Find the Index of the First Occurrence in a String (Easy)](https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/)
//!
//! Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

pub fn str_str(haystack: String, needle: String) -> i32 {
    // Could also just do:
    // haystack.find(&needle).map(|i| i as i32).unwrap_or(-1)

    // Instead of dual-pointer character compare, can use simple String equivalency check of a
    // sliding substring
    if haystack.len() >= needle.len() {
        for i in 0..=(haystack.len() - needle.len()) {
            if haystack[i..(i + needle.len())] == needle {
                return i as _;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();

        assert_eq!(str_str(haystack, needle), 0);
    }

    #[test]
    fn case_2() {
        let haystack = "leetcode".to_string();
        let needle = "leeto".to_string();

        assert_eq!(str_str(haystack, needle), -1);
    }
}
