//! # [3264. Final Array State After K Multiplication Operations I](https://leetcode.com/problems/final-array-state-after-k-multiplication-operations-i/description/)
//!
//! You are given an integer array nums, an integer k, and an integer multiplier. You need to perform k operations on nums. In each operation: Find the minimum value x in nums. If there are multiple occurrences of the minimum value, select the one that appears first. Replace the selected minimum value x with x * multiplier. Return an integer array denoting the final state of nums after performing all k operations.
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Naive method of just iterating k-times over, but simple and no extra allocation needed, runs in
/// O(N*k) time
pub fn get_final_state_naive(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut result = nums;
    for _ in 0..k {
        let mut min_idx = 0;
        let mut min = result[min_idx];
        for j in 0..result.len() {
            if result[j] < min {
                min_idx = j;
                min = result[j];
            }
        }
        result[min_idx] *= multiplier;
    }
    result
}

/// Use a binary heap of tuple (value, index), such that we can easily find the smallest value
pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let len = nums.len();
    let mut heap = BinaryHeap::with_capacity(len);

    // Push the value and index into heap, though note we wrap in Reverse() to create a
    // Min-heap (as opposed to default priority queue)
    for (i, &num) in nums.iter().enumerate() {
        // Need to wrap both values of tuple
        heap.push((Reverse(num), Reverse(i)));
    }

    for _ in 0..k {
        let (Reverse(val), Reverse(idx)) = heap.pop().unwrap();
        nums[idx] = val * multiplier;
        heap.push((Reverse(nums[idx]), Reverse(idx)));
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [2, 1, 3, 5, 6].to_vec();
        let k = 5;
        let multiplier = 2;
        assert_eq!(
            get_final_state(nums, k, multiplier),
            [8, 4, 6, 5, 6].to_vec()
        );
    }

    #[test]
    fn case_2() {
        let nums = [1, 2].to_vec();
        let k = 3;
        let multiplier = 4;
        assert_eq!(get_final_state(nums, k, multiplier), [16, 8].to_vec());
    }
}
