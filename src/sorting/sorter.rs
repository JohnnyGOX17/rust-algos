//! Sorter trait for common sorting algorithm implementation (makes integration test easy and
//! consistent across algos)

/// Common sorting trait
pub trait Sorter<T: Ord + Copy> {
    /// Sorting algorithm name
    fn name(&self) -> &'static str;

    /// Concrete sorting implementation
    fn sort(&self, arr: &mut [T]);
}
