pub const DAY_STR: &str = "day_05";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{self, newline},
        multi::{many1, separated_list1},
        sequence::preceded,
        IResult,
    };

    pub fn parse_seeds(input: &str) -> IResult<&str, Vec<u64>> {
        let (input, seeds) =
            preceded(tag("seeds: "), separated_list1(tag(" "), complete::u64))(input)?;
        Ok((input, seeds))
    }

    #[derive(Debug)]
    pub struct Map {
        pub source: u64,
        pub destination: u64,
        pub range: u64,
    }

    fn parse_map_line(input: &str) -> IResult<&str, Map> {
        let (input, destination) = complete::u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, source) = complete::u64(input)?;
        let (input, _) = tag(" ")(input)?;
        let (input, range) = complete::u64(input)?;

        Ok((
            input,
            Map {
                source,
                destination,
                range,
            },
        ))
    }

    fn parse_map(input: &str) -> IResult<&str, Vec<Map>> {
        let (input, _) = take_until("map:")(input)?;
        let (input, _) = tag("map:")(input)?;
        let (input, _) = newline(input)?;

        let (input, map) = separated_list1(newline, parse_map_line)(input)?;

        // dbg!(&map);
        // dbg!(&input);
        Ok((input, map))
    }

    pub fn parse_maps(input: &str) -> IResult<&str, Vec<Vec<Map>>> {
        let (input, maps) = many1(parse_map)(input)?;
        Ok((input, maps))
    }

    pub fn solution(input: String) -> u64 {
        let (input, seeds) = parse_seeds(&input).expect("failed to parse seeds");
        // dbg!(&seeds);

        let (_input, maps) = parse_maps(input).expect("failed for parse first map");
        // dbg!(&maps);

        let x = seeds
            .iter()
            .map(|seed| {
                let mut seed = *seed;
                // dbg!("New seed", seed);
                for map in &maps {
                    // dbg!("  ");
                    // dbg!(&map);
                    for element in map {
                        if let Some(offset) = seed.checked_sub(element.source) {
                            // dbg!(offset);
                            if offset < element.range {
                                seed = offset + element.destination;
                                // dbg!("YES");
                                // dbg!(seed);
                                break;
                            }
                        }
                    }
                    // dbg!(seed);
                    // dbg!(seed);
                    // todo!()
                }
                // dbg!("Final value", seed);
                seed
            })
            .min()
            .unwrap();

        // todo!()
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            35
        );
    }
}

mod part_2 {

    use std::mem::swap;

    use super::part_1::parse_maps;
    use super::part_1::parse_seeds;
    use super::part_1::Map;

    #[derive(Debug, Clone, Copy)]
    struct SeedRange {
        start: u64,
        end: u64,
    }

    #[derive(Debug, Clone, Copy)]
    struct TransformedSeedRangeSuccessType {
        transformed: SeedRange,
        excluded_before: Option<SeedRange>,
        excluded_after: Option<SeedRange>,
    }

    impl Map {
        fn source_end(&self) -> u64 {
            self.source + self.range
        }
        fn destination_end(&self) -> u64 {
            self.destination + self.range
        }
        fn source_contains(&self, val: u64) -> bool {
            self.source <= val && val < self.source_end()
        }

        fn translate_seed_range(&self, sr: &SeedRange) -> Option<TransformedSeedRangeSuccessType> {
            let m_start = self.source;
            let m_end = self.source_end();
            let s_start = sr.start;
            let s_end = sr.end;

            // let t_offset = self.source - self.destination;

            // dbg!(m_start, m_end, s_start, s_end);

            // leaking both
            if s_start < m_start && m_end < s_end {
                return Some(TransformedSeedRangeSuccessType {
                    transformed: SeedRange {
                        start: m_start - self.source + self.destination,
                        end: m_end - self.source + self.destination,
                    },
                    excluded_before: Some(SeedRange {
                        start: s_start,
                        end: m_start,
                    }),
                    excluded_after: Some(SeedRange {
                        start: m_end,
                        end: s_end,
                    }),
                });
            }

            // leaking after
            if m_start <= s_start && s_start < m_end && m_end < s_end {
                return Some(TransformedSeedRangeSuccessType {
                    transformed: SeedRange {
                        start: s_start - self.source + self.destination,
                        end: m_end - self.source + self.destination,
                    },
                    excluded_before: None,
                    excluded_after: Some(SeedRange {
                        start: m_end,
                        end: s_end,
                    }),
                });
            }

            // leaking before
            if s_start < m_start && m_start < s_end && s_end <= m_end {
                return Some(TransformedSeedRangeSuccessType {
                    transformed: SeedRange {
                        start: m_start - self.source + self.destination,
                        end: s_end - self.source + self.destination,
                    },
                    excluded_before: Some(SeedRange {
                        start: s_start,
                        end: m_start,
                    }),
                    excluded_after: None,
                });
            }

            // contained
            if m_start <= s_start && s_end <= m_end {
                return Some(TransformedSeedRangeSuccessType {
                    transformed: SeedRange {
                        start: s_start - self.source + self.destination,
                        end: s_end - self.source + self.destination,
                    },
                    excluded_before: None,
                    excluded_after: None,
                });
            }

            None

            // todo!("THis should never be reached")
        }
    }

    pub fn solution(input: String) -> u64 {
        let (input, seeds) = parse_seeds(&input).expect("failed to parse seeds");
        // dbg!(&seeds);

        let seeds = seeds
            .chunks(2)
            .map(|se| SeedRange {
                start: se[0],
                end: se[0] + se[1] - 1,
            })
            .collect::<Vec<_>>();
        // dbg!(&seeds);

        let (_input, maps) = parse_maps(input).expect("failed for parse first map");
        // dbg!(&maps);

        let mut seed_vec_a: Vec<SeedRange> = seeds.clone();
        let mut seed_vec_b: Vec<SeedRange> = vec![];

        let old_seeds = &mut seed_vec_a;
        let new_seeds = &mut seed_vec_b;
        swap(old_seeds, new_seeds);

        for map in &maps {
            swap(old_seeds, new_seeds);
            for m in map {
                // let seeds_to_check = old_seeds.clone();
                let mut skipped_seeds: Vec<SeedRange> = vec![];

                old_seeds.iter().for_each(|os| {
                    // dbg!(&os);
                    assert!(os.start < os.end);
                });

                while let Some(seed) = old_seeds.pop() {
                    // dbg!(&m);
                    // dbg!(&seed);
                    let x = m.translate_seed_range(&seed);
                    // dbg!(&seed);
                    // dbg!(x);
                    match x {
                        Some(res) => {
                            new_seeds.push(res.transformed);
                            if let Some(eb) = res.excluded_before {
                                skipped_seeds.push(eb);
                            }
                            if let Some(ea) = res.excluded_after {
                                skipped_seeds.push(ea);
                            }
                        }
                        None => skipped_seeds.push(seed),
                    }
                }

                // dbg!(&old_seeds);
                // dbg!(&skipped_seeds);
                // dbg!(&new_seeds);

                *old_seeds = skipped_seeds;
            }

            // dbg!(&old_seeds);

            new_seeds.append(old_seeds);
            old_seeds.clear();
            // dbg!(&new_seeds);

            // todo!()
        }

        // dbg!(&new_seeds);

        let x = new_seeds.iter().map(|ns| ns.start).min().unwrap();

        // todo!();
        x
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            46
        );
    }

    #[test]
    fn contains() {
        let map = Map {
            source: 55,
            destination: 50,
            range: 2,
        };

        let s = SeedRange { start: 55, end: 67 };

        let x = map.translate_seed_range(&s);
        dbg!(x);
    }
}
