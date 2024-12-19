//! # [9. Palindrome Number (Easy)](https://leetcode.com/problems/palindrome-number/description/)
//!
//! Given an integer x, return true if x is a palindrome , and false otherwise.

/// Simply convert number to string- reverse and compare, actually quicker than numeric method below!
pub fn is_palindrome(x: i32) -> bool {
    // negative sign also causes it to never be a palindrome
    if x < 0 {
        return false;
    }

    let reverse_str: String = x.to_string().chars().rev().collect();
    x.to_string() == reverse_str
}

pub fn is_palindrome_numeric(x: i32) -> bool {
    if x < 0 {
        // negative sign also causes it to never be a palindrome
        false
    } else if x < 10 {
        // single number, always palindrome
        true
    } else {
        // instead of converting to string (or array of chars), we can do this numberically
        // in a similar fashion, so find number of digits first
        let num_digits = (x as f64).log10().floor() as usize + 1;
        let mut num_stack: Vec<i32> = Vec::new();

        let mut mod_idx = num_digits as i32;
        for _ in 0..num_digits / 2 {
            // use modulo to isolate a certain set of digits
            let mod_mask = 10.0f64.powi(mod_idx) as i32;
            let div_mask = 10.0f64.powi(mod_idx - 1) as i32;

            let digit = (x % mod_mask) / div_mask;
            num_stack.push(digit);

            mod_idx -= 1;
        }

        // if odd, subtract once more, middle val not needed
        if num_digits % 2 == 1 {
            mod_idx -= 1;
        }

        for _ in 0..num_digits / 2 {
            // use modulo to isolate a certain set of digits
            let mod_mask = 10.0f64.powi(mod_idx) as i32;
            let div_mask = 10.0f64.powi(mod_idx - 1) as i32;

            let digit = (x % mod_mask) / div_mask;
            let match_digit = num_stack.pop();
            if match_digit != Some(digit) {
                // digit did not match or we hit a None!
                return false;
            }

            mod_idx -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn case_2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn case_3() {
        assert!(!is_palindrome(10));
    }

    #[test]
    fn case_4() {
        assert!(is_palindrome(4321234));
    }

    #[test]
    fn case_5() {
        assert!(is_palindrome(55));
    }
}
