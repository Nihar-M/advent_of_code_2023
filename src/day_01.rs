pub const DAY_STR: &str = "day_01";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    pub fn solution(input: String) -> u32 {
        let x = input
            .lines()
            .map(|line| {
                let first = line
                    .chars()
                    .find(|x| x.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let second = line
                    .chars()
                    .rev()
                    .find(|x| x.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                first * 10 + second
            })
            .collect::<Vec<_>>();
        // println!("{:?}", x);
        return x.iter().sum::<u32>();
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            142
        );
    }
}

mod part_2 {

    pub fn solution(input: String) -> usize {
        // let x = input.split_whitespace().collect::<Vec<_>>();
        let x = input
            .lines()
            .map(|f| {
                let words = vec![
                    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four",
                    "five", "six", "seven", "eight", "nine",
                ];

                // println!("{:?}", words);

                let res = words
                    .iter()
                    .flat_map(|word| f.match_indices(word).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

                // println!("{:?}", res);
                let first = res.iter().min_by_key(|(pos, _val)| pos).unwrap();
                let last = res.iter().max_by_key(|(pos, _val)| pos).unwrap();
                // println!("{:?}, {:?}", first, last);

                let mut f_num = words.iter().position(|x| x == &first.1).unwrap();
                let mut l_num = words.iter().position(|x| x == &last.1).unwrap();

                if f_num > 8 {
                    f_num -= 9;
                }
                f_num += 1;

                if l_num > 8 {
                    l_num -= 9;
                }
                l_num += 1;

                // println!("{:?}, {:?}", f_num, l_num);

                f_num * 10 + l_num
            })
            .collect::<Vec<_>>();
        // println!("{x:?}");
        x.iter().sum::<usize>()
    }

    fn reverse_string(s: &str) -> String {
        s.chars().rev().collect::<String>()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            281
        );
    }
}
