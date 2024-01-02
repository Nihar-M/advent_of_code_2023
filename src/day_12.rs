pub const DAY_STR: &str = "day_12";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{fmt::Debug, iter};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Spring {
        Operational,
        Damaged,
        Unknown,
    }

    pub struct DBGWrap<'a>(pub &'a [Spring]);
    impl<'a> Debug for DBGWrap<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = self.0.iter().map(|s| char::from(*s)).collect::<String>();
            f.write_str(&data)
        }
    }

    impl From<char> for Spring {
        fn from(value: char) -> Self {
            match value {
                '.' => Self::Operational,
                '#' => Self::Damaged,
                '?' => Self::Unknown,
                _x => panic!("Encounter unexpected character `{_x}`"),
            }
        }
    }

    impl From<Spring> for char {
        fn from(value: Spring) -> Self {
            match value {
                Spring::Operational => '.',
                Spring::Damaged => '#',
                Spring::Unknown => '?',
            }
        }
    }

    // checks if `length` of damaged springs can be place
    // at index **1** of the `map` slice.
    // requires a slice of at least size `length + 2`
    pub fn is_placeable(map: &[Spring], length: usize) -> bool {
        if map.len() < length + 2 {
            return false;
        }

        if map[0] == Spring::Damaged {
            return false;
        }

        if (1..=length).any(|i| map[i] == Spring::Operational) {
            return false;
        }

        if map[length + 1] == Spring::Damaged {
            return false;
        }

        true
    }

    pub fn arrangements(map: &mut [Spring], list: &[usize]) -> usize {
        // dbg!("new func");

        let mut num_arrangements = 0;
        let length = list[0];

        for start_idx in 0..map.len() {
            let partial = &map[start_idx..];
            let before = &map[..start_idx];
            // dbg!(DBGWrap(before));

            if before.len() > 1 && before.last().unwrap() == &Spring::Damaged {
                // dbg!("break");
                break;
            }

            // dbg!(DBGWrap(partial));

            let res = is_placeable(partial, length);
            // dbg!(res);

            if res {
                if list.len() > 1 {
                    let sub_map = &mut map[start_idx + length..];

                    let og_val = sub_map[0];
                    sub_map[0] = Spring::Damaged;

                    // dbg!("after placement");
                    // dbg!(DBGWrap(sub_map));
                    num_arrangements += arrangements(sub_map, &list[1..]);

                    sub_map[0] = og_val;

                    // dbg!(num_arrangements);
                } else {
                    // check if the second half has only Operational left

                    // dbg!(DBGWrap(partial));
                    let remaining = &partial[length + 1..];
                    // dbg!(DBGWrap(remaining));

                    if !remaining.iter().any(|s| s == &Spring::Damaged) {
                        num_arrangements += 1;
                        // dbg!(num_arrangements);
                    }

                    // todo!()
                }
            }
        }
        // dbg!(num_arrangements);
        // todo!();
        num_arrangements
    }

    #[test]
    fn test_arrangements_1() {
        use Spring::Damaged as D;
        use Spring::Operational as O;
        use Spring::Unknown as U;
        assert_eq!(
            arrangements(&mut [O, U, U, U, O, D, D, D, O], &[1, 1, 3]),
            1
        );
    }

    #[test]
    fn test_arrangements_2() {
        use Spring::Damaged as D;
        use Spring::Operational as O;
        use Spring::Unknown as U;

        assert_eq!(
            arrangements(&mut [O, U, U, U, O, D, O, D, O], &[1, 1, 1]),
            3
        );
    }

    pub fn solution(input: String) -> usize {
        let lines = input
            .lines()
            .map(|line| {
                let (map_str, list_str) = line.split_once(' ').unwrap();
                let map = iter::once(Spring::Operational)
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Operational))
                    .collect::<Vec<_>>();
                let list = list_str
                    .split_terminator(',')
                    .map(|s| s.parse::<usize>().expect("Failed to parse into `usize`"))
                    .collect::<Vec<_>>();
                (map, list)
            })
            .collect::<Vec<_>>();

        // dbg!(&x);

        let y = lines
            .into_iter()
            .map(|(mut map, list)| arrangements(&mut map, &list))
            .collect::<Vec<_>>();

        // dbg!(&y);

        // let z = arrangements(&mut y.0, &y.1);

        y.iter().sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            21
        );
    }
}

mod part_2 {

    use std::iter;

    use cached::proc_macro::cached;

    use super::part_1::*;

    #[cached]
    pub fn arrangements(mut map: Vec<Spring>, list: Vec<usize>) -> usize {
        let mut num_arrangements = 0;
        let length = list[0];

        for start_idx in 0..map.len() {
            // check if first half has only Operational
            let partial = &map[start_idx..];

            // if start_idx > 0 {

            // }

            let before = &map[..start_idx];
            // dbg!(DBGWrap(before));

            if before.len() > 1 && before.last().unwrap() == &Spring::Damaged {
                // dbg!("break");
                break;
            }

            // dbg!(DBGWrap(partial));

            let res = is_placeable(partial, length);
            // dbg!(res);

            if res {
                if list.len() > 1 {
                    let sub_map = &mut map[start_idx + length..];

                    let og_val = sub_map[0];
                    sub_map[0] = Spring::Damaged;

                    // dbg!("after placement");
                    // dbg!(DBGWrap(sub_map));
                    num_arrangements += arrangements(sub_map.to_vec(), (list[1..]).to_vec());

                    sub_map[0] = og_val;

                    // dbg!(num_arrangements);
                } else {
                    // check if the second half has only Operational left
                    // dbg!(DBGWrap(partial));

                    let remaining = &partial[length + 1..];
                    // dbg!(DBGWrap(remaining));

                    if !remaining.iter().any(|s| s == &Spring::Damaged) {
                        num_arrangements += 1;
                        // dbg!(num_arrangements);
                    }

                    // todo!()
                }
            }
        }
        // dbg!(num_arrangements);
        // todo!();
        num_arrangements
    }

    #[test]
    fn test_arrangements_1() {
        // use Spring::Damaged as D;
        // use Spring::Operational as O;
        // use Spring::Unknown as U;

        let x = ".???.###????.###????.###????.###????.###."
            .chars()
            .map(Spring::from)
            .collect::<Vec<_>>();

        let y = vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3];

        assert_eq!(arrangements(x, y), 1);
    }

    #[test]
    fn test_arrangements_2() {
        // use Spring::Damaged as D;
        // use Spring::Operational as O;
        // use Spring::Unknown as U;

        let x = "..??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.?.??..??...?##.."
            .chars()
            .map(Spring::from)
            .collect::<Vec<_>>();

        let y = vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3];

        assert_eq!(arrangements(x, y), 16384);
    }

    pub fn solution(input: String) -> usize {
        let lines = input
            .lines()
            .map(|line| {
                let (map_str, list_str) = line.split_once(' ').unwrap();

                let map = iter::once(Spring::Operational)
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Unknown))
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Unknown))
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Unknown))
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Unknown))
                    .chain(map_str.chars().map(Spring::from))
                    .chain(iter::once(Spring::Operational))
                    .collect::<Vec<_>>();

                let mut list = list_str
                    .split_terminator(',')
                    .map(|s| s.parse::<usize>().expect("Failed to parse into `usize`"))
                    .collect::<Vec<_>>();

                let old_list = list.clone();
                for _ in 0..4 {
                    list.append(&mut old_list.clone());
                }

                // for l in &list {
                //     print!("{},", l);
                // }
                // println!();
                // dbg!(DBGWrap(&map));

                (map, list)
            })
            // .map(|(mut map, mut list)| {
            //     for _ in 0..4 {
            //         map.push(Spring::Unknown);
            //         map.append(&mut map.clone());
            //         list.append(&mut list.clone());
            //     }
            //     (map, list)
            // })
            .collect::<Vec<_>>();

        // dbg!(&lines);

        let y = lines
            .into_iter()
            .map(|(map, list)| arrangements(map, list))
            .collect::<Vec<_>>();

        // dbg!(&y);

        // let z = arrangements(&mut y.0, &y.1);

        y.iter().sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            525152
        );
    }
}
