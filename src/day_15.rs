pub const DAY_STR: &str = "day_15";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use itertools::Itertools;
    use rstest::*;

    pub fn hash_alg(input: String) -> u8 {
        // unknown if doing a wrapping_add is allowed?

        input
            .chars()
            .fold(0, |h, c| (h.wrapping_add(c as u8)).wrapping_mul(17))
    }

    #[rstest]
    #[case("", 0)]
    #[case("H", 200)]
    #[case("HA", 153)]
    #[case("HAS", 172)]
    #[case("HASH", 52)]
    fn test_hash_alg(#[case] input: String, #[case] result: u8) {
        assert_eq!(hash_alg(input), result);
    }

    pub fn solution(input: String) -> u32 {
        let z = input
            .split_terminator(',')
            .map(|s| hash_alg(s.to_string()))
            .collect_vec();
        // dbg!(&z);

        z.into_iter().map(|n| n as u32).sum::<u32>()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            1320
        );
    }
}

mod part_2 {

    use std::collections::HashMap;

    use itertools::Itertools;

    use super::part_1::hash_alg;

    pub fn solution(input: String) -> usize {
        let commands = input
            .split_terminator(',')
            .map(|s| {
                // dbg!(s);
                if s.contains('=') {
                    let (name, focal_length) = s.split_terminator('=').collect_tuple().unwrap();
                    // dbg!(name);
                    // dbg!(focal_length);
                    (name.to_string(), Some(focal_length))
                } else {
                    let (name,) = s.split_terminator('-').collect_tuple().unwrap();
                    // dbg!(name);
                    (name.to_string(), None)
                }
            })
            // .map(|s| hash_alg(s.to_string()))
            .collect_vec();

        let mut boxes: HashMap<u8, Vec<(String, u32)>> = HashMap::new();
        // dbg!(&commands);

        for (name, focal_len) in commands {
            let box_hash = hash_alg(name.to_string());

            if let Some(focal_len) = focal_len {
                let focal_len = focal_len.parse::<u32>().unwrap();

                if let Some(vector) = &mut boxes.get_mut(&box_hash) {
                    if let Some((_n, fl)) = vector.iter_mut().find(|(n, _fl)| n == &name) {
                        *fl = focal_len;
                    } else {
                        vector.push((name, focal_len))
                    }
                } else {
                    boxes.insert(box_hash, vec![(name, focal_len)]);
                }
            } else if let Some(vector) = boxes.get_mut(&box_hash) {
                let remove_idx =
                    vector
                        .iter()
                        .enumerate()
                        .find_map(|(idx, (n, _fl))| if n == &name { Some(idx) } else { None });
                if let Some(remove_idx) = remove_idx {
                    vector.remove(remove_idx);
                }
            }

            // dbg!(&boxes);
        }

        boxes
            .into_iter()
            .map(|(box_idx, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(idx, (_n, fl))| (idx + 1) * (*fl as usize))
                    .sum::<usize>()
                    * (box_idx as usize + 1)
            })
            .sum::<usize>()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            145
        );
    }
}
