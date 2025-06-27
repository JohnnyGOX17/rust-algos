//! # Insertion Sort
//!
//! ![img](https://upload.wikimedia.org/wikipedia/commons/4/42/Insertion_sort.gif)
//!
//! Sorts a mutable slice in-place by:
//! - At each increasing index, exchanging the entries that are smaller to the left.
//! - As the entries to the left are sorted at each iteration, the array is fully sorted once the
//!   index reaches the right end.
//!
//! Performance:
//! - Time complexity is $O(n^{2})$
//! - Space complexity is $O(1)$ (sorts in-place)
use crate::sorting::sorter::Sorter;

pub struct InsertionSort;

impl<T: Ord + Copy> Sorter<T> for InsertionSort {
    fn name(&self) -> &'static str {
        "InsertionSort"
    }

    fn sort(&self, arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            let cur = arr[i];

            while j > 0 && cur < arr[j - 1] {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            arr[j] = cur;
        }
    }
}
