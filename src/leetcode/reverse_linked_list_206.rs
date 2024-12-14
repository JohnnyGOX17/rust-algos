//! # [206. Reverse Linked List (Easy)](https://leetcode.com/problems/reverse-linked-list/description/)
//!
//! Given the `head` of a singly linked list, reverse the list, and return the reversed list.
//! Example 1:
//!  Input: `head = [1,2,3,4,5]`
//!  Output: `[5,4,3,2,1]`
//! Constraints:
//!  * The number of nodes in the list is in the range `[0, 5000]`.
//!  * `-5000 <= Node.val <= 5000`

/// Definition for singly-linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));

        let expected_out = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 1, next: None })),
                    })),
                })),
            })),
        }));
        assert_eq!(expected_out, reverse_list(input));
    }
}
