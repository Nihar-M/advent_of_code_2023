pub const DAY_STR: &str = "day_11";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use itertools::Itertools;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Empty,
        Galaxy,
    }

    fn display_space(space: &[Vec<Tile>]) {
        space.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Galaxy => print!("#"),
                };
            });
            println!();
        })
    }

    pub fn solution(input: String) -> usize {
        let mut space = input
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Galaxy,
                        _ => panic!("Cannot convert `{c}` into a `Tile`"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // display_space(&space);

        let empty_horizontal = space
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if row.iter().all(|t| *t == Tile::Empty) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // let col_length = space.len();
        let row_length = space.first().unwrap().len();

        // dbg!(&empty_horizontal);

        empty_horizontal
            .iter()
            .rev()
            .for_each(|empty_idx| space.insert(*empty_idx, vec![Tile::Empty; row_length]));

        // display_space(&space);

        let empty_vertical = (0..row_length)
            .filter(|col_idx| {
                let x = space.iter().fold(true, |is_empty, row| {
                    if row[*col_idx] == Tile::Empty {
                        is_empty
                    } else {
                        false
                    }
                });
                // dbg!(x);
                x
            })
            .collect::<Vec<_>>();

        // dbg!(&empty_vertical);

        empty_vertical.iter().rev().for_each(|empty_idx| {
            // dbg!(empty_idx);

            space.iter_mut().for_each(|row| {
                row.insert(*empty_idx, Tile::Empty);
            });
        });

        // display_space(&space);

        let galaxies = space
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if *tile == Tile::Galaxy {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        // dbg!(&galaxies);

        let dist = galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let a = pair[0];
                let b = pair[1];

                let d_x = a.0.abs_diff(b.0);
                let d_y = a.1.abs_diff(b.1);
                d_x + d_y
            })
            .collect::<Vec<_>>();

        // dbg!(&dist);

        let total_distance = dist.iter().sum::<usize>();

        total_distance

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            374
        );
    }
}

mod part_2 {

    use std::mem::swap;

    use itertools::Itertools;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Tile {
        Galaxy,
        Empty,
        VerticalExpand,
        HorizontalExpand,
        AllExpand,
    }

    fn display_space(space: &[Vec<Tile>]) {
        space.iter().for_each(|row| {
            row.iter().for_each(|tile| {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Galaxy => print!("#"),
                    Tile::VerticalExpand => print!("|"),
                    Tile::HorizontalExpand => print!("-"),
                    Tile::AllExpand => print!("+"),
                };
            });
            println!();
        })
    }

    pub fn solution(input: String) -> usize {
        let mut space = input
            .lines()
            .map(|row| {
                row.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Galaxy,
                        _ => panic!("Cannot convert `{c}` into a `Tile`"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // display_space(&space);

        let empty_horizontal = space
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if row.iter().all(|t| *t == Tile::Empty) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // let col_length = space.len();
        let row_length = space.first().unwrap().len();

        // dbg!(&empty_horizontal);

        empty_horizontal.iter().for_each(|empty_idx| {
            // dbg!(empty_idx);
            space[*empty_idx]
                .iter_mut()
                .for_each(|tile| *tile = Tile::HorizontalExpand);
        });

        // display_space(&space);

        let empty_vertical = (0..row_length)
            .filter(|col_idx| {
                let x = space.iter().fold(true, |is_empty, row| {
                    if row[*col_idx] == Tile::Galaxy {
                        false
                    } else {
                        is_empty
                    }
                });
                // dbg!(x);
                x
            })
            .collect::<Vec<_>>();

        // dbg!(&empty_vertical);

        empty_vertical.iter().for_each(|empty_idx| {
            // dbg!(empty_idx);
            space.iter_mut().for_each(|row| {
                row[*empty_idx] = match row[*empty_idx] {
                    Tile::Empty => Tile::VerticalExpand,
                    Tile::HorizontalExpand => Tile::AllExpand,
                    _ => panic!(),
                }
            });
        });

        // display_space(&space);

        let galaxies = space
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if *tile == Tile::Galaxy {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>();

        // dbg!(&galaxies);

        let expansion_factor = 1000000;

        let dist = galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let &(mut ax, mut ay) = pair[0];
                let &(mut bx, mut by) = pair[1];

                // dbg!(ax, ay, bx, by);

                if bx < ax {
                    swap(&mut ax, &mut bx);
                }

                if by < ay {
                    swap(&mut ay, &mut by);
                }

                let jumps_horizontal = empty_vertical
                    .iter()
                    .filter(|expansion_line| {
                        // dbg!(ax, expansion_line, bx);
                        ax <= **expansion_line && **expansion_line <= bx
                    })
                    .collect::<Vec<_>>();

                let jumps_vertical = empty_horizontal
                    .iter()
                    .filter(|expansion_line| {
                        // dbg!(ay, expansion_line, by);
                        ay <= **expansion_line && **expansion_line <= by
                    })
                    .collect::<Vec<_>>();

                // dbg!(&jumps_horizontal, &jumps_vertical);

                let jumps_horizontal = jumps_horizontal.len();
                let jumps_vertical = jumps_vertical.len();

                let d_x = ax.abs_diff(bx) + jumps_horizontal * (expansion_factor - 1);
                let d_y = ay.abs_diff(by) + jumps_vertical * (expansion_factor - 1);

                // dbg!(distance);
                d_x + d_y
            })
            .collect::<Vec<_>>();

        // dbg!(&dist);

        let total_distance = dist.iter().sum::<usize>();

        total_distance

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            82000210
        );
    }
}
