pub const DAY_STR: &str = "day_22";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{collections::HashSet, fmt::Debug};

    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, newline},
        multi::separated_list1,
        sequence::{separated_pair, tuple},
        IResult,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Pos3 {
        pub x: u64,
        pub y: u64,
        pub z: u64,
    }

    impl Debug for Pos3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "Pos3( x : {} , y : {}, z : {} )",
                self.x, self.y, self.z
            ))
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Cube {
        pub lower: Pos3,
        pub upper: Pos3,
    }

    impl Cube {
        pub fn intersects(self, other: Cube) -> bool {
            !(self.upper.z < other.lower.z
                || self.lower.z > other.upper.z
                || self.upper.y < other.lower.y
                || self.lower.y > other.upper.y
                || self.upper.x < other.lower.x
                || self.lower.x > other.upper.x)
        }
    }

    #[test]
    fn test_intersects() {
        let a = Cube {
            lower: Pos3 {
                x: 10,
                y: 10,
                z: 10,
            },
            upper: Pos3 {
                x: 20,
                y: 20,
                z: 20,
            },
        };

        let mut b = Cube {
            lower: Pos3 {
                x: 10,
                y: 10,
                z: 10,
            },
            upper: Pos3 {
                x: 20,
                y: 20,
                z: 20,
            },
        };

        assert!(a.intersects(b));
        b.lower.z = 0;
        assert!(a.intersects(b));
        b.upper.z = 5;
        assert!(!a.intersects(b));
        b.upper.z = 25;
        assert!(a.intersects(b));
    }

    pub fn parse_input(input: &str) -> IResult<&str, Vec<Cube>> {
        let (input, semi_parsed) = separated_list1(
            newline,
            separated_pair(
                tuple((
                    complete::u64,
                    tag(","),
                    complete::u64,
                    tag(","),
                    complete::u64,
                )),
                tag("~"),
                tuple((
                    complete::u64,
                    tag(","),
                    complete::u64,
                    tag(","),
                    complete::u64,
                )),
            ),
        )(input)?;

        Ok((
            input,
            semi_parsed
                .into_iter()
                .map(|(lower, upper)| Cube {
                    lower: Pos3 {
                        x: lower.0, // lower.1 = ","
                        y: lower.2, // lower.3 = ","
                        z: lower.4,
                    },
                    upper: Pos3 {
                        x: upper.0, // upper.1 = ","
                        y: upper.2, // upper.3 = ","
                        z: upper.4,
                    },
                })
                .collect_vec(),
        ))
        // todo!()
    }

    pub fn solution(input: String) -> usize {
        let (_input, mut falling) = parse_input(&input).unwrap();

        // cubes.iter().for_each(|cube| {
        //     assert!(cube.lower.x <= cube.upper.x);
        //     assert!(cube.lower.y <= cube.upper.y);
        //     assert!(cube.lower.z <= cube.upper.z);
        // });

        // dbg!(&cubes);
        falling.sort_by_cached_key(|cube| -(cube.lower.z as i64));

        // dbg!(&falling);

        let mut settled = vec![];

        let supported_by = falling
            .clone()
            .into_iter()
            .rev()
            .map(|mut cube| loop {
                // dbg!(cube);
                cube.lower.z -= 1;
                cube.upper.z -= 1;
                let supports = settled
                    .iter()
                    .rev()
                    .filter(|other| cube.intersects(**other))
                    .copied()
                    .collect_vec();
                // dbg!(&supports);
                if cube.lower.z == 0 || !supports.is_empty() {
                    cube.lower.z += 1;
                    cube.upper.z += 1;
                    settled.push(cube);
                    break supports;
                }
            })
            .collect_vec();

        // dbg!(&supported_by);

        // dbg!(&settled);

        let key_cubes: HashSet<Cube> = HashSet::from_iter(supported_by.iter().filter_map(|sups| {
            if sups.len() == 1 {
                Some(sups[0])
            } else {
                None
            }
        }));

        // dbg!(&key_cubes);

        falling.len() - key_cubes.len()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            5
        );
    }
}

mod part_2 {

    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;

    use super::part_1::*;

    fn disintegrate(
        cubes: &mut HashSet<Cube>,
        supported_by: &mut HashMap<Cube, HashSet<Cube>>,
        is_supporting: &mut HashMap<Cube, HashSet<Cube>>,
        cube: Cube,
    ) -> HashSet<Cube> {
        // dbg!(&cubes);
        // dbg!(cube);
        assert!(cubes.remove(&cube));

        if let Some(supporting) = is_supporting.remove(&cube) {
            // dbg!(&supporting);
            for supported in supporting {
                // dbg!(&supported);
                let supports = supported_by.get_mut(&supported).expect("Should work?");
                supports.remove(&cube);
                // dbg!(&supports);
            }
        }

        // dbg!(&supported_by);

        let unsupported = supported_by
            .iter()
            .filter_map(|(x_cube, x_supports)| {
                if x_supports.is_empty() {
                    Some(*x_cube)
                } else {
                    None
                }
            })
            .collect_vec();

        for un_sup in &unsupported {
            supported_by.remove(un_sup);
        }

        // dbg!(&unsupported);

        let mut disintegrated: HashSet<Cube> = unsupported
            .iter()
            .map(|x_cube| disintegrate(cubes, supported_by, is_supporting, *x_cube))
            .fold(HashSet::new(), |set, cube_set| {
                set.union(&cube_set).copied().collect::<HashSet<Cube>>()
            });

        disintegrated.insert(cube);
        disintegrated
    }

    pub fn solution(input: String) -> usize {
        let (_input, mut falling) = parse_input(&input).unwrap();

        // cubes.iter().for_each(|cube| {
        //     assert!(cube.lower.x <= cube.upper.x);
        //     assert!(cube.lower.y <= cube.upper.y);
        //     assert!(cube.lower.z <= cube.upper.z);
        // });

        // dbg!(&cubes);
        falling.sort_by_cached_key(|cube| -(cube.lower.z as i64));

        // dbg!(&falling);

        let ground_cube = Cube {
            lower: Pos3 { x: 0, y: 0, z: 0 },
            upper: Pos3 {
                x: 1000,
                y: 1000,
                z: 0,
            },
        };

        let mut settled = vec![];

        let supported_by: HashMap<Cube, HashSet<Cube>> =
            HashMap::from_iter(falling.clone().into_iter().rev().map(|mut cube| loop {
                // dbg!(cube);
                cube.lower.z -= 1;
                cube.upper.z -= 1;
                let supports: HashSet<Cube> = HashSet::from_iter(
                    settled
                        .iter()
                        .rev()
                        .filter(|other| cube.intersects(**other))
                        .copied(),
                );
                // dbg!(&supports);
                if cube.lower.z == 0 {
                    cube.lower.z += 1;
                    cube.upper.z += 1;
                    settled.push(cube);
                    break (cube, HashSet::from_iter(vec![ground_cube]));
                } else if !supports.is_empty() {
                    cube.lower.z += 1;
                    cube.upper.z += 1;
                    settled.push(cube);
                    break (cube, supports);
                }
            }));

        let cubes: HashSet<Cube> = HashSet::from_iter(settled.clone());
        // dbg!(&settled);

        // dbg!(&supported_by);

        let is_supporting: HashMap<Cube, HashSet<Cube>> =
            supported_by
                .iter()
                .fold(HashMap::new(), |mut is_supporting, (cube, supports)| {
                    supports.iter().for_each(|support| {
                        if let Some(supporting) = is_supporting.get_mut(support) {
                            supporting.insert(*cube);
                        } else {
                            is_supporting.insert(*support, HashSet::from_iter(vec![*cube]));
                        }
                    });
                    is_supporting
                });

        // dbg!(&is_supporting);

        let others = cubes
            .iter()
            .map(|starting_cube| {
                let mut cubes = cubes.clone();
                let mut supported_by = supported_by.clone();
                let mut is_supporting = is_supporting.clone();

                let disintegrated = disintegrate(
                    &mut cubes,
                    &mut supported_by,
                    &mut is_supporting,
                    *starting_cube,
                );
                // dbg!(disintegrated);
                disintegrated.len() - 1
            })
            .collect_vec();

        // dbg!(&others);

        others.iter().sum()
        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            7
        );
    }
}
