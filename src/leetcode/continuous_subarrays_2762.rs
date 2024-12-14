//! # 2762. Continuous Subarrays (Medium)
//!
//! You are given a 0-indexed integer array nums. A subarray of nums is called continuous if:
//! Let `i, i + 1, ..., j` be the indices in the subarray. Then, for each pair of indices `i <= i1`, `i2 <= j`, `0 <= |nums[i1] - nums[i2]| <= 2`.
//! Return the total number of continuous subarrays.
//! A subarray is a contiguous non-empty sequence of elements within an array.

/// Do an optimized two pointer
pub fn continuous_subarrays(nums: Vec<i32>) -> i64 {
    let mut left = 0;
    let mut right = 0;
    let mut j = 0;

    let mut count = 0;

    for i in 0..nums.len() {
        if i == 0 {
            // first element
            count += 1;
            left = nums[i] - 2; // range from a - 2
            right = nums[i] + 2; // range from a + 2
        } else if nums[i] >= left && nums[i] <= right {
            // If new element belongs to same range
            // then
            left = left.max(nums[i] - 2); // take max from (a-2, b-2)
            right = right.min(nums[i] + 2); // take min from (a+2, b+2)
            count += i - j + 1; // add subarray to answer
        } else {
            // If doesn't belong to ranges try to remove from i-1
            j = i - 1;
            // set new ranges
            left = nums[i] - 2;
            right = nums[i] + 2;
            while nums[j] >= nums[i] - 2 && nums[j] <= nums[i] + 2 {
                left = left.max(nums[j] - 2); // take max
                right = right.min(nums[j] + 2); // take min
                j -= 1;
            }
            j += 1;
            count += i - j + 1; // add subarray to answer
        }
    }

    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = [5, 4, 2, 4].to_vec();
        assert_eq!(continuous_subarrays(input), 8);
    }

    #[test]
    fn case_2() {
        let input = [1, 2, 3].to_vec();
        assert_eq!(continuous_subarrays(input), 6);
    }
}
