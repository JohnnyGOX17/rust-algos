//! # [1. Two Sum (Easy)](https://leetcode.com/problems/two-sum/description/)
//!
//! Given an array of integers `nums` and an integer `target`, return
//! _indices of the two numbers such that they add up to `target`_.
//!
//! You may assume that each input would have **exactly one solution**,
//! and you may not use the _same_ element twice.
//!
//! You can return the answer in any order.
use std::collections::HashMap;

/// Use a one-pass hash table to perform in O(n) time: while iterating and inserting elements into
/// the hash table, look back at each iteration by searching hash table for complementary value. If
/// value exists, return indices immediately.
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut sum_indices = Vec::<i32>::with_capacity(2);

    for (i, val) in nums.iter().enumerate() {
        // complementary value for current value
        let complement_val = target - val;

        // If the complementary value (used as hashmap index here) is found, instantly return with
        // both indices
        if let Some(found_idx) = map.get(&complement_val) {
            sum_indices.push(*found_idx);
            sum_indices.push(i as i32);
            return sum_indices;
        }
        // Else not found, insert current value in hashmap as key with index as value
        map.insert(*val, i as i32);
    }

    // Return empty vector if not found
    sum_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [2, 7, 11, 15].to_vec();
        let target = 9;
        assert_eq!(two_sum(nums, target), [0, 1].to_vec());
    }

    #[test]
    fn case_2() {
        let nums = [3, 2, 4].to_vec();
        let target = 6;
        assert_eq!(two_sum(nums, target), [1, 2].to_vec());
    }

    #[test]
    fn case_3() {
        let nums = [3, 3].to_vec();
        let target = 6;
        assert_eq!(two_sum(nums, target), [0, 1].to_vec());
    }
}
