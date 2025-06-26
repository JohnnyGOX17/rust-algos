//! # [295. Find Median from Data Stream (Hard)](https://leetcode.com/problems/find-median-from-data-stream/description/)
//!
//! The median is the middle value in an ordered integer list. If the size of the list is even,
//! there is no middle value, and the median is the mean of the two middle values. For example, for
//! `arr = [2,3,4]`, the median is 3. For example, for `arr = [2,3]`, the median is (2 + 3) / 2 = 2.5.
//! Implement the MedianFinder class: MedianFinder() initializes the MedianFinder object. void
//! addNum(int num) adds the integer num from the data stream to the data structure. double
//! findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer
//! will be accepted.

#![allow(dead_code)]
use std::collections::BinaryHeap;

struct MedianFinder {
    /// Max-heap to sort as we insert, as left -> median -> right, pop() will return largest value
    /// next to median
    left_heap: BinaryHeap<i32>,

    /// Min-heap to sort as we insert, as left -> median -> right, pop() will return smallest value
    /// next to median
    right_heap: BinaryHeap<i32>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    /// From https://www.geeksforgeeks.org/median-of-stream-of-integers-running-integers/
    fn add_num(&mut self, num: i32) {
        self.left_heap.push(num);
        let mut temp = self.left_heap.pop().unwrap();

        self.right_heap.push(-temp);
        if self.right_heap.len() > self.left_heap.len() {
            temp = self.right_heap.pop().unwrap();
            self.left_heap.push(-temp);
        }
    }

    fn find_median(&self) -> f64 {
        if self.left_heap.len() != self.right_heap.len() {
            *self.left_heap.peek().unwrap() as f64
        } else {
            (*self.left_heap.peek().unwrap() as f64 - *self.right_heap.peek().unwrap() as f64) / 2.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut obj = MedianFinder::new();
        obj.add_num(1);
        obj.add_num(2);
        assert_eq!(obj.find_median(), 1.5);
    }
}
