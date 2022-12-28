pub struct Solution;
use std::{cmp::Reverse, collections::BinaryHeap, ops::AddAssign};

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let heap: BinaryHeap<_> = capacity
            .into_iter()
            .zip(rocks)
            .map(|(c, r)| Reverse(c - r))
            .collect();

        heap.into_draining()
            .map(|v| v.0)
            .cumsum::<i32>()
            .take_while(|csum| *csum <= additional_rocks)
            .count() as i32
    }
}

// Impl into_draining() and cumsum() to avoid nightly features and ext crates

struct HeapDrainIterator<T>(BinaryHeap<T>);

impl<T> Iterator for HeapDrainIterator<T>
where
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

trait HeapDrainIteratorExt<T> {
    fn into_draining(self) -> HeapDrainIterator<T>;
}

impl<T> HeapDrainIteratorExt<T> for BinaryHeap<T> {
    fn into_draining(self) -> HeapDrainIterator<T> {
        HeapDrainIterator(self)
    }
}

struct Cumsum<'a, I, Sum>
where
    I: Iterator,
{
    iter: &'a mut I,
    acc: Sum,
}

impl<'a, I, Sum> Iterator for Cumsum<'a, I, Sum>
where
    I: Iterator,
    Sum: AddAssign<I::Item> + Copy,
{
    type Item = Sum;

    fn next(&mut self) -> Option<Self::Item> {
        self.acc += self.iter.next()?;
        Some(self.acc)
    }
}

trait CumsumIteratorExt<I: Iterator> {
    fn cumsum<'a, Sum>(&'a mut self) -> Cumsum<'a, I, Sum>
    where
        Sum: AddAssign<I::Item> + Default;
}

impl<I: Iterator> CumsumIteratorExt<I> for I {
    fn cumsum<'a, Sum>(&'a mut self) -> Cumsum<'a, I, Sum>
    where
        Sum: AddAssign<I::Item> + Default,
    {
        Cumsum {
            iter: self,
            acc: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,4,5], vec![1,2,4,4], 2, 3)]
    #[case(vec![10,2,2], vec![2,2,0], 100, 3)]
    #[case(vec![54,18,91,49,51,45,58,54,47,91,90,20,85,20,90,49,10,84,59,29,40,9,100,1,64,71,30,46,91], vec![14,13,16,44,8,20,51,15,46,76,51,20,77,13,14,35,6,34,34,13,3,8,1,1,61,5,2,15,18], 77, 13)]
    fn bags_test(
        #[case] capacity: Vec<i32>,
        #[case] rocks: Vec<i32>,
        #[case] additional_rocks: i32,
        #[case] expected: i32,
    ) {
        assert_eq!(
            Solution::maximum_bags(capacity, rocks, additional_rocks),
            expected
        );
    }
}
