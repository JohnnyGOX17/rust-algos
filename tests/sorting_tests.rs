//! # Integration tests for Sorting Algorithms

use rust_algos::sorting::{insertion_sort, sorter::Sorter};

fn get_all_sorters() -> Vec<Box<dyn Sorter<i32>>> {
    vec![Box::new(insertion_sort::InsertionSort)]
}

#[test]
fn test_sorting_algos() {
    let inputs = vec![
        vec![3, 1, 4, 2],
        vec![5],
        vec![],
        vec![2, 2, 2],
        vec![9, 7, 5, 3, 1],
    ];

    for mut input in inputs {
        let expected = {
            let mut cloned = input.clone();
            cloned.sort(); // use standard sorting algo
            cloned
        };

        for sort_algo in get_all_sorters() {
            sort_algo.sort(&mut input);
            assert_eq!(input, expected, "Failed on sorter: {}", sort_algo.name());
        }
    }
}
