//! # [2182. Construct String With Repeat Limit (Medium)](https://leetcode.com/problems/construct-string-with-repeat-limit/description/)
//!
//! You are given a string s and an integer repeatLimit. Construct a new string repeatLimitedString
//! using the characters of s such that no letter appears more than repeatLimit times in a row. You
//! do not have to use all characters from s. Return the lexicographically largest
//! repeatLimitedString possible. A string a is lexicographically larger than a string b if in the
//! first position where a and b differ, string a has a letter that appears later in the alphabet
//! than the corresponding letter in b. If the first min(a.length, b.length) characters do not
//! differ, then the longer string is the lexicographically larger one.
use std::collections::BinaryHeap;

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    // Binary heap used to order letters as they are inserted
    let mut letter_heap = BinaryHeap::new();
    // Backup queue to push characters to when breaking up repeat limits
    let mut backup_queue = Vec::new();

    let mut ret_str = String::with_capacity(s.len());

    // Initially fill heap, and given default priority-queue model, the largest char's will pop() first in order
    for char in s.chars() {
        letter_heap.push(char);
    }

    let mut letter_count = 1;
    let mut prev_char = '~'; // assume ok since s is only lowercase English letters (e.g. always no match on first iteration)

    while !letter_heap.is_empty() {
        let curr_char = letter_heap.pop().unwrap(); // should always have something of the two

        if curr_char == prev_char {
            letter_count += 1;
        } else {
            letter_count = 1;
        }

        if letter_count <= repeat_limit {
            println!("Pushed to out: {}", curr_char);
            ret_str.push(curr_char);
            prev_char = curr_char;
        } else {
            letter_count = 1;
            let mut push_char = Some(curr_char);
            println!("hit limit");
            loop {
                backup_queue.push(push_char.unwrap());
                push_char = letter_heap.pop();
                if push_char.is_none() {
                    return ret_str;
                } else if push_char.unwrap() < curr_char {
                    println!("Pushed to out(from inner): {}", push_char.unwrap());
                    ret_str.push(push_char.unwrap());
                    prev_char = push_char.unwrap();
                    // flush backup_queue
                    while let Some(element) = backup_queue.pop() {
                        letter_heap.push(element);
                    }
                    break;
                }
            }
        }
    }

    ret_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let s = "cczazcc".to_string();
        let repeat_limit = 3;
        assert_eq!(repeat_limited_string(s, repeat_limit), "zzcccac");
    }

    #[test]
    fn case_2() {
        let s = "aababab".to_string();
        let repeat_limit = 2;
        assert_eq!(repeat_limited_string(s, repeat_limit), "bbabaa");
    }

    #[test]
    fn case_3() {
        let s = "robnsdvpuxbapuqgopqvxdrchivlifeepy".to_string();
        let repeat_limit = 2;
        assert_eq!(
            repeat_limited_string(s, repeat_limit),
            "yxxvvuvusrrqqppopponliihgfeeddcbba"
        );
    }
}
