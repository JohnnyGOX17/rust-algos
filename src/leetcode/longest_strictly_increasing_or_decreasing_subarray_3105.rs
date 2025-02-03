//! # [3105. Longest Strictly Increasing or Strictly Decreasing Subarray (Easy)](https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/description/)
//!
//! You are given an array of integers nums. Return the length of the longest subarray of nums
//! which is either strictly increasing or strictly decreasing.

pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let mut inc_cntr = 1;
    let mut dec_cntr = 1;
    let mut last_val = -10; // actual val between 1:50
    let mut longest_len = 1;

    for (i, &val) in nums.iter().enumerate() {
        if last_val < val && i > 0 {
            inc_cntr += 1;
        } else {
            longest_len = longest_len.max(inc_cntr);
            inc_cntr = 1;
        }

        if last_val > val && i > 0 {
            dec_cntr += 1;
        } else {
            longest_len = longest_len.max(dec_cntr);
            dec_cntr = 1;
        }

        last_val = val;
    }

    // Check for longest on last index
    longest_len = longest_len.max(dec_cntr);
    longest_len.max(inc_cntr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [1, 4, 3, 3, 2].to_vec();
        assert_eq!(longest_monotonic_subarray(nums), 2);
    }

    #[test]
    fn case_2() {
        let nums = [3, 3, 3, 3].to_vec();
        assert_eq!(longest_monotonic_subarray(nums), 1);
    }

    #[test]
    fn case_3() {
        let nums = [3, 2, 1].to_vec();
        assert_eq!(longest_monotonic_subarray(nums), 3);
    }
}
