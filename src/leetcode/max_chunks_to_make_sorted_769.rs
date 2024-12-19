//! # [769. Max Chunks To Make Sorted (Medium)](https://leetcode.com/problems/max-chunks-to-make-sorted/description/)
//!
//! You are given an integer array arr of length n that represents a permutation of the integers in
//! the range [0, n - 1]. We split arr into some number of chunks (i.e., partitions), and
//! individually sort each chunk. After concatenating them, the result should equal the sorted
//! array. Return the largest number of chunks we can make to sort the array.

/// Use max element approach. Here, we iterate through the array while keeping track of the maximum
/// element we've encountered up to the current index.
pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    // Consider the case where the current index i, is equal to the maximum element encountered so
    // far, maxElement. This condition means that all elements preceding index i are less than
    // maxElement. Since the array is a permutation of integers in the range [0,n−1], it also
    // guarantees that all integers from 0 to maxElement must appear in the array before index i.
    // Therefore, whenever the current index matches the maximum value so far (i.e.,
    // i==maxElement), we increment the count of chunks.
    let mut num_chunks = 0;
    let mut max_element = 0i32;

    for (i, &val) in arr.iter().enumerate() {
        max_element = val.max(max_element);
        if max_element == i as i32 {
            num_chunks += 1;
        }
    }

    num_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let arr = [4, 3, 2, 1, 0].to_vec();
        assert_eq!(max_chunks_to_sorted(arr), 1);
    }

    #[test]
    fn case_2() {
        let arr = [1, 0, 2, 3, 4].to_vec();
        assert_eq!(max_chunks_to_sorted(arr), 4);
    }
}
