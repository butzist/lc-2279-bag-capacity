struct Solution;

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let mut remaining: Vec<_> = capacity
            .into_iter()
            .zip(rocks)
            .map(|(c, r)| c - r)
            .collect();

        remaining.sort();

        let mut acc: i32 = 0;
        remaining
            .into_iter()
            .map(|v| {
                // no cumsum in stdlib, yet
                acc += v;
                acc
            })
            .take_while(|csum| *csum <= additional_rocks)
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use rstest::rstest;

    #[rstest]
    #[case(vec![2,3,4,5], vec![1,2,4,4], 2, 3)]
    #[case(vec![10,2,2], vec![2,2,0], 100, 3)]
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
