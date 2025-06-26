pub trait Sorter<T: Ord + Copy> {
    fn name(&self) -> &'static str;

    fn sort(&self, arr: &mut [T]);
}
