//! #[2. Add Two Numbers (Medium)](https://leetcode.com/problems/add-two-numbers/description/)
//!
//! You are given two non-empty linked lists representing two non-negative integers. The digits are
//! stored in reverse order, and each of their nodes contains a single digit. Add the two numbers
//! and return the sum as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//!Constraints:
//! * The number of nodes in each linked list is in the range `[1, 100]`.
//! * `0 <= Node.val <= 9`
//! * It is guaranteed that the list represents a number that does not have leading zeros.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/// Mimic carry logic to traverse each linked list (which is already ordered w/head being the least
/// significant digit) and successively add each decimal place and build a return linked list
pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1_ptr = &l1;
    let mut l2_ptr = &l2;
    let mut carry = 0;

    // create head node for return list
    let mut ret_head_node = Some(Box::new(ListNode::new(0)));
    let mut prev_node = ret_head_node.as_mut().unwrap();

    while l1_ptr.is_some() || l2_ptr.is_some() || carry != 0 {
        let mut dig1 = 0;
        if l1_ptr.is_some() {
            dig1 = l1_ptr.as_ref().unwrap().val;
            l1_ptr = &l1_ptr.as_ref().unwrap().next;
        }

        let mut dig2 = 0;
        if l2_ptr.is_some() {
            dig2 = l2_ptr.as_ref().unwrap().val;
            l2_ptr = &l2_ptr.as_ref().unwrap().next;
        }

        let sum = dig1 + dig2 + carry;
        let new_val = sum % 10;
        carry = sum / 10;

        prev_node.val = new_val;
        if l1_ptr.is_some() || l2_ptr.is_some() || carry != 0 {
            prev_node.next = Some(Box::new(ListNode::new(new_val)));
            prev_node = prev_node.next.as_mut().unwrap();
        }
    }

    ret_head_node
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds linked list from last element -> head (reverse order)
    fn arr_to_list(n: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for val in n {
            let mut new_node = ListNode::new(*val);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }
        head
    }

    #[test]
    fn case_1() {
        let l1 = arr_to_list(&[3, 4, 2]);
        let l2 = arr_to_list(&[4, 6, 5]);
        let expected = arr_to_list(&[8, 0, 7]);

        assert_eq!(add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn case_2() {
        let l1 = Some(Box::new(ListNode::new(0)));
        let l2 = Some(Box::new(ListNode::new(0)));
        let expected = Some(Box::new(ListNode::new(0)));

        assert_eq!(add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn case_3() {
        let l1 = arr_to_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = arr_to_list(&[9, 9, 9, 9]);
        let expected = arr_to_list(&[1, 0, 0, 0, 9, 9, 9, 8]);

        assert_eq!(add_two_numbers(l1, l2), expected);
    }

    #[test]
    fn case_4() {
        let l1 = arr_to_list(&[9]);
        let l2 = arr_to_list(&[9, 9, 9, 9, 9, 9, 9, 9, 9, 1]);
        let expected = arr_to_list(&[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

        assert_eq!(add_two_numbers(l1, l2), expected);
    }
}
