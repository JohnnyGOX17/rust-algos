//! # [27. Remove Element (Easy)](https://leetcode.com/problems/remove-element/description/)
//!
//! Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
//! The order of the elements may be changed. Then return the number of elements in nums which are
//! not equal to val. Consider the number of elements in nums which are not equal to val be k, to
//! get accepted, you need to do the following things: Change the array nums such that the first k
//! elements of nums contain the elements which are not equal to val. The remaining elements of
//! nums are not important as well as the size of nums. Return k.

/// Simply push non-matching values into new Vec, and return length of that vec and swap memory
/// values with original mutable input vec
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut ret_vec: Vec<i32> = Vec::new();
    for i_val in &mut *nums {
        if *i_val != val {
            ret_vec.push(*i_val);
        }
    }
    std::mem::swap(nums, &mut ret_vec);
    // number of elements in nums which are not equal to val
    nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums = [3, 2, 2, 3].to_vec();
        assert_eq!(remove_element(&mut nums, 3), 2);
        assert_eq!(nums[..2], [2, 2].to_vec());
    }

    #[test]
    fn case_2() {
        let mut nums = [0, 1, 2, 2, 3, 0, 4, 2].to_vec();
        assert_eq!(remove_element(&mut nums, 2), 5);
        assert_eq!(nums[..5], [0, 1, 3, 0, 4].to_vec());
    }
}
