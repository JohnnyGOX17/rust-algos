//! # [21. Merge Two Sorted Lists (Easy)](https://leetcode.com/problems/merge-two-sorted-lists/description/)
//!
//! You are given the heads of two sorted linked lists list1 and list2. Merge the two lists into
//! one sorted list. The list should be made by splicing together the nodes of the first two lists.
//! Return the head of the merged linked list.

/// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut retval = &mut list1;

    while list2.is_some() {
        if retval.is_none() || list2.as_ref()?.val < retval.as_ref()?.val {
            std::mem::swap(retval, &mut list2);
        }
        retval = &mut retval.as_mut()?.next;
    }
    list1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let list1 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));
        let list2 = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        }));

        let checklist = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            })),
        }));

        assert_eq!(merge_two_lists(list1, list2), checklist);
    }

    #[test]
    fn case_2() {
        let list1: Option<Box<ListNode>> = None;
        let list2: Option<Box<ListNode>> = None;
        assert_eq!(merge_two_lists(list1, list2), None);
    }

    #[test]
    fn case_3() {
        let list1: Option<Box<ListNode>> = None;
        let list2 = Some(Box::new(ListNode { val: 0, next: None }));
        let checklist = list2.clone();
        assert_eq!(merge_two_lists(list1, list2), checklist);
    }
}
