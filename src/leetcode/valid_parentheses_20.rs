//! # [20. Valid Parentheses (Easy)](https://leetcode.com/problems/valid-parentheses/description/)
//!
//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if
//! the input string is valid. An input string is valid if:
//!  1. Open brackets must be closed by the same type of brackets.
//!  2. Open brackets must be closed in the correct order.
//!  3. Every close bracket has a corresponding open bracket of the same type.
pub fn is_valid(s: String) -> bool {
    let in_bytes = s.as_bytes();
    // Use a basic Vec when wanting a basic stack:
    //  https://doc.rust-lang.org/std/collections/
    let mut paren_stack: Vec<char> = Vec::new();

    for &in_byte in in_bytes.iter() {
        let in_char = in_byte as char;

        match in_char {
            // On opening char, push complimentary closing char for easy comparison
            '(' => paren_stack.push(')'),
            '{' => paren_stack.push('}'),
            '[' => paren_stack.push(']'),
            _ => {
                // Only paren chars in input string, so can simply compare against popped value
                // Also return false if stack empty (closing char seen with no opening)
                if paren_stack.is_empty() || (paren_stack.pop() != Some(in_char)) {
                    return false;
                }
            }
        }
    }
    // If stack is empty, we have balanced parens
    paren_stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!(is_valid("()".to_string()));
    }

    #[test]
    fn case_2() {
        assert!(is_valid("()[]{}".to_string()));
    }

    #[test]
    fn case_3() {
        assert!(!is_valid("(]".to_string()));
    }
}
