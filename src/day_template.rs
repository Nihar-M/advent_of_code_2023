pub const DAY_STR: &str = "day_{}";

pub mod part_1 {

    pub fn solution(input: String) -> u32 {
        todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            1
        );
    }
}

pub mod part_2 {

    pub fn solution(input: String) -> usize {
        todo!()
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
