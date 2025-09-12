//! # [20. Valid Parentheses (Easy)](https://leetcode.com/problems/valid-parentheses/description/)
//!
//! Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if
//! the input string is valid. An input string is valid if:
//!  1. Open brackets must be closed by the same type of brackets.
//!  2. Open brackets must be closed in the correct order.
//!  3. Every close bracket has a corresponding open bracket of the same type.

/// Canonical example using simple `Vec` as a LIFO stack to check for balance
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

/// Alternative ultra-low/fixed memory version for very constrained/embedded systems by using a
/// fixed size array, instead of `Vec`, to avoid heap allocation (`#![no_std]` friendly).
pub fn is_valid_minimal(s: &str) -> bool {
    const MAX_DEPTH: usize = 32;
    let mut stack: [u8; MAX_DEPTH] = [0; MAX_DEPTH];
    let mut stack_ptr: usize = 0;

    for byte in s.bytes() {
        match byte {
            b'(' | b'{' | b'[' => {
                if stack_ptr >= MAX_DEPTH {
                    // we've hit the maximum paren depth, we have to fail here to not have stack
                    // overflow
                    return false;
                }
                stack[stack_ptr] = byte;
                stack_ptr += 1;
            }
            b')' => {
                if stack_ptr == 0 || stack[stack_ptr - 1] != b'(' {
                    return false;
                }
                stack_ptr -= 1;
            }
            b'}' => {
                if stack_ptr == 0 || stack[stack_ptr - 1] != b'{' {
                    return false;
                }
                stack_ptr -= 1;
            }
            b']' => {
                if stack_ptr == 0 || stack[stack_ptr - 1] != b'[' {
                    return false;
                }
                stack_ptr -= 1;
            }
            // Change this to ignore other characters if input string is expected to have more than
            // parentheses in it.
            _ => return false,
        }
    }

    stack_ptr == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_cases() {
        assert!(is_valid("()".to_string()));
        assert!(is_valid("()[]{}".to_string()));
        assert!(is_valid("{[]}".to_string()));
        assert!(is_valid("((()))".to_string()));

        assert!(is_valid_minimal("()"));
        assert!(is_valid_minimal("()[]{}"));
        assert!(is_valid_minimal("{[]}"));
        assert!(is_valid_minimal("((()))"));
    }

    #[test]
    fn test_invalid_cases() {
        assert!(!is_valid("(]".to_string()));
        assert!(!is_valid("([)]".to_string())); // interleaved case
        assert!(!is_valid("((".to_string()));
        assert!(!is_valid("())".to_string()));

        assert!(!is_valid_minimal("(]"));
        assert!(!is_valid_minimal("([)]")); // interleaved case
        assert!(!is_valid_minimal("(("));
        assert!(!is_valid_minimal("())"));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_valid("".to_string()));
        assert!(!is_valid("a".to_string()));

        assert!(is_valid_minimal(""));
        assert!(!is_valid_minimal("a"));
    }
}
