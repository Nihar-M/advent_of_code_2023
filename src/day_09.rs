pub const DAY_STR: &str = "day_09";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    pub fn vector_diff(pattern: &Vec<i32>) -> Vec<i32> {
        assert!(pattern.len() > 2);
        pattern.windows(2).map(|x| x[1] - x[0]).collect()
    }

    #[test]
    fn test_diff() {
        let x = vec![1, 3, 6, 10, 15, 21];
        let d = vec![2, 3, 4, 5, 6];
        assert_eq!(vector_diff(&x), d)
    }

    pub fn solution(input: String) -> i32 {
        let patterns = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().expect("Failed to parse into i32"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // dbg!(patterns);

        let next_values = patterns
            .into_iter()
            .map(|pattern| {
                // dbg!(&pattern);
                let mut diffs = vec![];
                diffs.push(pattern);
                while !diffs.last().unwrap().iter().all(|x| *x == 0) {
                    diffs.push(vector_diff(diffs.last().unwrap()))
                }
                // dbg!(&diffs);

                // let mut last = diffs.pop().unwrap().last().unwrap();

                diffs.into_iter().rev().fold(0, |next_diff, pattern| {
                    // dbg!(next_diff);
                    let last = pattern.last().unwrap();
                    // dbg!(last);
                    last + next_diff
                })
            })
            .collect::<Vec<_>>();

        // dbg!(&next_values);
        next_values.into_iter().sum()

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            114
        );
    }
}

mod part_2 {

    use super::part_1::vector_diff;

    pub fn solution(input: String) -> i32 {
        let patterns = input
            .lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| x.parse::<i32>().expect("Failed to parse into i32"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // dbg!(&patterns);

        let next_values = patterns
            .into_iter()
            .map(|pattern| {
                // dbg!(&pattern);
                let mut diffs = vec![];
                diffs.push(pattern);
                while !diffs.last().unwrap().iter().all(|x| *x == 0) {
                    diffs.push(vector_diff(diffs.last().unwrap()))
                }
                // dbg!(&diffs);

                // let mut last = diffs.pop().unwrap().last().unwrap();

                diffs.into_iter().rev().fold(0, |previous_diff, pattern| {
                    // dbg!(previous_diff);
                    let first = pattern.first().unwrap();
                    // dbg!(first);
                    first - previous_diff
                })
            })
            .collect::<Vec<_>>();

        // dbg!(&next_values);
        next_values.into_iter().sum()

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            2
        );
    }
}
