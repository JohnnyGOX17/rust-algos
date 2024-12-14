//! Binary Search (or half-interval/logarithmic search) is a search algorithm that finds the
//! position of a target value within a sorted array. It compares the target value to the middle of
//! the array. If the target value is greater than the middle value, search the middle of the lower
//! half. If the target value is lesser than the middle value, search the middle of the upper half.
//! Repeat until the target value is found, or no elements are left! Assumes data is sorted in
//! ascending order. Matches behavior of [slice primitive's binary_search](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) method.
//!
//! | Time Complexity | Space Complexity |
//! | --------------- | ---------------- |
//! | O(log n)        | O(1)             |
//!
//! ![binary_search wikipedia image](https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Binary_Search_Depiction.svg/2560px-Binary_Search_Depiction.svg.png)

use std::cmp::Ordering;

pub fn binary_search<T: Ord>(target: &T, arr: &[T]) -> Option<usize> {
    binary_search_recursive(target, arr, &0usize, &arr.len())
}

/// Recursive implementation of above
pub fn binary_search_recursive<T: Ord>(
    target: &T,
    arr: &[T],
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        None
    } else {
        let mid = left + (right - left) / 2;

        match target.cmp(&arr[mid]) {
            Ordering::Greater => binary_search_recursive(target, arr, &(mid + 1), right),
            Ordering::Equal => Some(mid),
            Ordering::Less => binary_search_recursive(target, arr, left, &mid),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&"a", &[]);
        assert_eq!(index, None);
    }

    #[test]
    fn one_item() {
        let index = binary_search(&"a", &["a"]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn search_strings_asc() {
        let index = binary_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"google", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(4));
    }

    #[test]
    fn search_ints_asc() {
        let index = binary_search(&4, &[1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &[1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &[1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &[1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &[1, 2, 3, 4]);
        assert_eq!(index, None);

        let index = binary_search(&5, &[4, 3, 2, 1]);
        assert_eq!(index, None);
    }
}
