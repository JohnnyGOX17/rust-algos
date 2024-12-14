//! # 2540. Minimum Common Value (Easy)
//!
//! <https://leetcode.com/problems/minimum-common-value/description/>
//!
//! Given two integer arrays nums1 and nums2, sorted in non-decreasing order, return the minimum
//! integer common to both arrays. If there is no common integer amongst nums1 and nums2, return -1.
//! Note that an integer is said to be common to nums1 and nums2 if both arrays have at least one
//! occurrence of that integer.
//!Constraints:
//!  * `1 <= nums1.length, nums2.length <= 10^5`
//!  * `1 <= nums1[i], nums2[j] <= 10^9`
//!  * Both nums1 and nums2 are sorted in non-decreasing order.
use std::cmp::Ordering;

/// In this method, use a two pointer approach as opposed to brute force method of searching across
/// both arrays to find a common value, which would be O(n*m). In this way, no extra space is
/// required (a couple variables but nothing that grows with input size so space complexity of
/// O(1)) and run in time complexity O(n+m).
///
/// This method works due to both input arrays being sorted:
///  1. When the value pointed to by the first pointer is less than the value pointed to by the
///     second pointer, we know that every element in the second vector after that pointer will
///     also be greater than the current first vector's pointed value. Thus we increment the
///     first pointer to the next index.
///  2. When the value pointed to by the second pointer is less than the value pointed to by the
///     first pointer, the exact inverse logic of above is also true.
///  3. If however both pointed to values are equal, we found a minimum match, so return!
/// This method also allows for iteration over unequal vector lengths and essentially latches the
/// final value of the smaller vector when it reaches the end of the array before the other one.
///
/// Another method is to use binary search pattern; since both arrays are sorted
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut n1_ptr = 0;
    let mut n2_ptr = 0;

    while (n1_ptr < nums1.len()) && (n2_ptr < nums2.len()) {
        let n1 = nums1[n1_ptr];
        let n2 = nums2[n2_ptr];

        match n1.cmp(&n2) {
            Ordering::Less => n1_ptr += 1,
            Ordering::Greater => n2_ptr += 1,
            Ordering::Equal => return n1,
        }
    }

    // No common integer found
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums1 = [1, 2, 3].to_vec();
        let nums2 = [2, 4].to_vec();
        assert_eq!(get_common(nums1, nums2), 2);
    }

    #[test]
    fn case_2() {
        let nums1 = [1, 2, 3, 6].to_vec();
        let nums2 = [2, 3, 4, 5].to_vec();
        assert_eq!(get_common(nums1, nums2), 2);
    }

    #[test]
    fn case_3() {
        let nums1 = [1, 2, 3, 7, 8].to_vec();
        let nums2 = [4, 5, 7, 8, 9].to_vec();
        assert_eq!(get_common(nums1, nums2), 7);
    }

    #[test]
    fn case_4() {
        let nums1 = [
            11, 15, 28, 31, 36, 42, 46, 54, 58, 63, 64, 67, 75, 76, 76, 79, 83, 85, 87, 90,
        ]
        .to_vec();
        let nums2 = [
            3, 6, 8, 13, 15, 19, 22, 25, 29, 29, 32, 35, 43, 43, 48, 55, 81, 90, 91, 94,
        ]
        .to_vec();
        assert_eq!(get_common(nums1, nums2), 15);
    }

    #[test]
    fn case_last_val() {
        let nums1 = [1, 2, 3, 7].to_vec();
        let nums2 = [4, 5, 7, 9].to_vec();
        assert_eq!(get_common(nums1, nums2), 7);
    }

    #[test]
    fn case_no_common() {
        let nums1 = [1, 2, 3].to_vec();
        let nums2 = [4, 4, 6].to_vec();
        assert_eq!(get_common(nums1, nums2), -1);
    }
}
