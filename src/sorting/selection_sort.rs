//! # Selection Sort
//!
//! ![img](https://upload.wikimedia.org/wikipedia/commons/9/94/Selection-Sort-Animation.gif)
//!
//! One of the simplest sorting algorithms:
//! 1. Find the smallest item in the array and then exchange it with the first entry.
//! 2. Then find the second smallest entry and exchange with second entry.
//! 3. Continue until the entire array is sorted.
//!
//! Because it needs to scan the entire array before making a placement decision on each iteration,
//! it needs $O(N^{2})$ runtime, irregardless of input sorting (e.g. even if input was already
//! sorted).
//!
//! Performance:
//! - Time complexity is $O(n^{2})$
//! - Space complexity is $O(1)$ (sorts in-place)

use crate::sorting::sorter::Sorter;

pub struct SelectionSort;

impl<T: Ord + Copy> Sorter<T> for SelectionSort {
    fn name(&self) -> &'static str {
        "SelectionSort"
    }

    fn sort(&self, arr: &mut [T]) {
        for i in 0..arr.len() {
            let mut min_index = i;
            for j in i + 1..arr.len() {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
                arr.swap(i, min_index);
            }
        }
    }
}
