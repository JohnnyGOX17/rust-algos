//! # [13. Roman to Integer (Easy)](https://leetcode.com/problems/roman-to-integer/description/)
//!
//! Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//! Symbol       Value
//! I             1
//! V             5
//! X             10
//! L             50
//! C             100
//! D             500
//! M             1000
//!
//! For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written
//! as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II. Roman
//! numerals are usually written largest to smallest from left to right. However, the numeral for
//! four is not IIII. Instead, the number four is written as IV. Because the one is before the five
//! we subtract it making four. The same principle applies to the number nine, which is written as
//! IX. There are six instances where subtraction is used: I can be placed before V (5) and X (10)
//! to make 4 and 9. X can be placed before L (50) and C (100) to make 40 and 90. C can be placed
//! before D (500) and M (1000) to make 400 and 900. Given a roman numeral, convert it to an
//! integer.

fn char_to_num(c: char) -> i32 {
    match c {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => 0,
    }
}

pub fn roman_to_int(s: String) -> i32 {
    let chars = s.to_uppercase(); // ensure uppercase
    let mut total = 0;
    let mut last_val = 99999; // make sure doesn't trigger on first run
    for letter in chars.chars() {
        let num = char_to_num(letter);

        // hit the case where we handle something like IV == 4
        if last_val < num {
            total -= last_val; // back out last value
            total += num - last_val; // use updated value
        } else {
            total += num;
        }

        last_val = num;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn case_3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
