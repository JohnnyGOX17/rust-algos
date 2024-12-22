//! # [26. Remove Duplicates from Sorted Array](https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/)
//!
//! Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such
//! that each unique element appears only once. The relative order of the elements should be kept
//! the same. Then return the number of unique elements in nums. Consider the number of unique
//! elements of nums to be k, to get accepted, you need to do the following things: Change the
//! array nums such that the first k elements of nums contain the unique elements in the order they
//! were present in nums initially. The remaining elements of nums are not important as well as the
//! size of nums. Return k.

/// Use two pointer method
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut j = 1; // unique index + len counter
    for i in 1..nums.len() {
        // if not ==, we encountered a new element
        if nums[i] != nums[i - 1] {
            // so update entry at unique index j with new value, overwriting that previously
            // duplicate value
            nums[j] = nums[i];
            // increment to move to next position
            j += 1;
        }
    }
    // unique index is same as overall unique length
    j as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = [1, 1, 2].to_vec();
        let expected_len: usize = 2;
        assert_eq!(remove_duplicates(&mut nums), expected_len as i32);
        assert_eq!(nums[..expected_len], [1, 2].to_vec());
    }

    #[test]
    fn case_2() {
        let mut nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4].to_vec();
        let expected_len: usize = 5;
        assert_eq!(remove_duplicates(&mut nums), expected_len as i32);
        assert_eq!(nums[..expected_len], [0, 1, 2, 3, 4].to_vec());
    }
}
