//! #[3423. Maximum Difference Between Adjacent Elements in a Circular Array (Easy)](https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/)
//!
//! Given a circular array `nums`, find the maximum absolute difference between adjacent elements.
//! **Note:** In a circular array, the first and last elements are adjacent.

pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let mut max_diff = 0;

    for i in 0..nums.len() - 1 {
        let diff = (nums[i] - nums[i + 1]).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }

    // last case for circular array
    let diff = (nums[nums.len() - 1] - nums[0]).abs();
    if diff > max_diff {
        max_diff = diff;
    }

    max_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [1, 2, 4].to_vec();
        assert_eq!(max_adjacent_distance(nums), 3);
    }

    #[test]
    fn case_2() {
        let nums = [-5, -10, -5].to_vec();
        assert_eq!(max_adjacent_distance(nums), 5);
    }
}
