//! # Insertion Sort

/// Sorts a mutable slice in-place:
/// - Time complexity is $O(n^{2})$
/// - Space complexity is $O(1)$ (sorts in-place)
pub fn insertion_sort<T: Ord + Copy>(arr: &mut [T]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut arr: Vec<&str> = vec!["d", "a", "c", "e", "b"];
        let expected: Vec<&str> = vec!["a", "b", "c", "d", "e"];
        insertion_sort(&mut arr);
        assert_eq!(arr, expected);
    }
}
