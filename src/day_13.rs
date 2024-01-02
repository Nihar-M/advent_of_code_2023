pub const DAY_STR: &str = "day_13";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use std::{cmp::min, usize};

    use itertools::Itertools;

    #[derive(Debug)]
    pub struct Image(pub Vec<String>);

    fn get_mirrored_indices(idx: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {
        let left_size = idx + 1;
        let right_size = size - idx - 1;
        // dbg!(left_size, right_size);

        let to_take = min(left_size, right_size);
        // dbg!(to_take);

        let left = (0..idx + 1).rev().take(to_take);
        let right = (idx + 1..size).take(to_take);

        // dbg!(left.collect_vec());
        // dbg!(right.collect_vec());

        left.zip(right)
    }

    #[test]
    fn mirrored_idx() {
        assert_eq!(
            get_mirrored_indices(4, 9).collect_vec(),
            vec![(4, 5), (3, 6), (2, 7), (1, 8)]
        );
        assert_eq!(
            get_mirrored_indices(2, 9).collect_vec(),
            vec![(2, 3), (1, 4), (0, 5)]
        );
    }

    impl Image {
        pub fn height(&self) -> usize {
            self.0.len()
        }

        pub fn width(&self) -> usize {
            self.0.first().unwrap().len()
        }

        pub fn check_mirrored_vertical(&self, idx: usize) -> bool {
            // dbg!(self);
            // dbg!(idx);

            get_mirrored_indices(idx, self.width()).all(|(l_idx, r_idx)| {
                // dbg!(l_idx);
                // dbg!(r_idx);
                let pair_mirrored = self
                    .0
                    .iter()
                    .all(|row| row.chars().nth(l_idx) == row.chars().nth(r_idx));
                // dbg!(&pair_mirrored);
                pair_mirrored
            })
        }

        pub fn count_defects_mirrored_vertical(&self, idx: usize) -> usize {
            // dbg!(self);
            // dbg!(idx);

            get_mirrored_indices(idx, self.width())
                .map(|(l_idx, r_idx)| {
                    // dbg!(l_idx);
                    // dbg!(r_idx);
                    let pair_defects = self
                        .0 // dbg!(mirrored);
                        .iter()
                        .map(|row| {
                            let l = row.chars().nth(l_idx);
                            let r = row.chars().nth(r_idx);
                            if l == r {
                                0
                            } else {
                                1
                            }
                        })
                        .sum::<usize>();
                    // dbg!(&pair_mirrored);
                    // dbg!(pair_defects);
                    pair_defects
                })
                .sum()
        }

        pub fn check_mirrored_horizontal(&self, idx: usize) -> bool {
            // dbg!(self);
            // dbg!(idx);

            get_mirrored_indices(idx, self.height()).all(|(l_idx, r_idx)| {
                // dbg!(l_idx);
                // dbg!(r_idx);
                let l_row = &self.0[l_idx];
                let r_row = &self.0[r_idx];

                l_row == r_row
            })
        }

        pub fn count_defects_mirrored_horizontal(&self, idx: usize) -> usize {
            // dbg!(self);
            // dbg!(idx);

            get_mirrored_indices(idx, self.height())
                .map(|(l_idx, r_idx)| {
                    // dbg!(l_idx);
                    // dbg!(r_idx);
                    let l_row = &self.0[l_idx];
                    let r_row = &self.0[r_idx];
                    let pair_mirrored = l_row
                        .chars()
                        .zip(r_row.chars())
                        .map(|(l, r)| if l == r { 0 } else { 1 })
                        .sum::<usize>();
                    // dbg!(&pair_mirrored);
                    pair_mirrored
                })
                .sum()
        }
    }

    pub fn solution(input: String) -> usize {
        let images = input
            .split_terminator("\n\n")
            .map(|group| {
                let y = group
                    .split_terminator('\n')
                    .map(|s| s.to_string())
                    .collect();
                // dbg!(&y);
                Image(y)
            })
            .collect_vec();
        // dbg!(&images);

        let mirror_sum = images
            .iter()
            .map(|image| {
                if let Some(v_i) =
                    (0..image.width() - 1).find(|i| image.check_mirrored_vertical(*i))
                {
                    // dbg!(v_i);
                    v_i + 1
                } else if let Some(h_i) =
                    (0..image.height() - 1).find(|i| image.check_mirrored_horizontal(*i))
                {
                    // dbg!(h_i);
                    (h_i + 1) * 100
                } else {
                    panic!();
                }
            })
            .collect_vec();

        // dbg!(&mirror_sum);

        // for i in 0..y.width() - 1 {
        //     y.check_mirrored_vertical(i);
        // }
        // for i in 0..y.height() - 1 {
        //     y.check_mirrored_horizontal(i);
        // }

        let x = mirror_sum.iter().sum::<usize>();
        // todo!();
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            405
        );
    }
}

mod part_2 {

    use itertools::Itertools;

    use super::part_1::Image;

    pub fn solution(input: String) -> usize {
        let images = input
            .split_terminator("\n\n")
            .map(|group| {
                let y = group
                    .split_terminator('\n')
                    .map(|s| s.to_string())
                    .collect();
                // dbg!(&y);
                Image(y)
            })
            .collect_vec();
        // dbg!(&images);

        let mirror_sum = images
            .iter()
            .map(|image| {
                if let Some(v_i) =
                    (0..image.width() - 1).find(|i| image.count_defects_mirrored_vertical(*i) == 1)
                {
                    // dbg!(v_i);
                    v_i + 1
                } else if let Some(h_i) = (0..image.height() - 1)
                    .find(|i| image.count_defects_mirrored_horizontal(*i) == 1)
                {
                    // dbg!(h_i);

                    (h_i + 1) * 100
                } else {
                    panic!();
                }
            })
            .collect_vec();

        // dbg!(&mirror_sum);
        let x = mirror_sum.iter().sum::<usize>();
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            400
        );
    }
}
