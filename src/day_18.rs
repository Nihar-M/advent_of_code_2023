pub const DAY_STR: &str = "day_18";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Dir {
        Up,
        Down,
        Left,
        Right,
    }

    pub fn show_grid(grid: &Vec<Vec<bool>>) {
        for row in grid {
            for c in row {
                print!("{}", if *c { '#' } else { '.' });
            }
            println!();
        }
    }

    pub fn solution(input: String) -> usize {
        use Dir::*;

        let trenches = input
            .lines()
            .map(|line| {
                let (dir, mag, _hex) = line.split_whitespace().collect_tuple().unwrap();
                let dir = match dir {
                    "U" => Up,
                    "D" => Down,
                    "L" => Left,
                    "R" => Right,
                    other => panic!("Unexpected direction {other}"),
                };
                let mag = mag.parse::<u32>().unwrap();
                (dir, mag)
            })
            .collect_vec();

        let range_ud = trenches
            .iter()
            .filter_map(|(dir, mag)| {
                if dir == &Down {
                    Some(*mag as i32 + 1)
                } else if dir == &Up {
                    Some(-(*mag as i32 + 1))
                } else {
                    None
                }
            })
            .scan(0, |state, mag| {
                *state += mag;
                Some(*state)
            })
            .collect_vec();

        let range_lr = trenches
            .iter()
            .filter_map(|(dir, mag)| {
                if dir == &Right {
                    Some(*mag as i32 + 1)
                } else if dir == &Left {
                    Some(-(*mag as i32 + 1))
                } else {
                    None
                }
            })
            .scan(0, |state, mag| {
                *state += mag;
                Some(*state)
            })
            .collect_vec();

        // dbg!(&range_ud);
        // dbg!(&range_lr);

        let n_rows = 1 + (range_ud.iter().max().unwrap() - range_ud.iter().min().unwrap()) as usize;
        let n_cols = 1 + (range_lr.iter().max().unwrap() - range_lr.iter().min().unwrap()) as usize;

        // dbg!(n_rows);
        // dbg!(n_cols);

        let mut y = -range_ud.iter().min().unwrap() as usize;
        let mut x = -range_lr.iter().min().unwrap() as usize;

        // dbg!(x);
        // dbg!(y);

        let mut grid = vec![vec![false; n_cols]; n_rows];

        grid[y][x] = true;

        for (dir, mag) in trenches {
            // show_grid(&grid);
            // dbg!(dir);
            // dbg!(mag);
            for _ in 0..mag {
                match dir {
                    Up => y -= 1,
                    Down => y += 1,
                    Left => x -= 1,
                    Right => x += 1,
                }
                grid[y][x] = true;
            }
        }

        // show_grid(&grid);

        let mut visited = vec![vec![false; n_cols]; n_rows];
        let mut fringe = vec![(0, 0)];

        while let Some((sy, sx)) = fringe.pop() {
            visited[sy][sx] = true;

            if sx > 0 {
                let nx = sx - 1;
                let ny = sy;
                if !visited[ny][nx] && !grid[ny][nx] {
                    fringe.push((ny, nx));
                }
            }

            if sx < n_cols - 1 {
                let nx = sx + 1;
                let ny = sy;
                if !visited[ny][nx] && !grid[ny][nx] {
                    fringe.push((ny, nx));
                }
            }

            if sy > 0 {
                let nx = sx;
                let ny = sy - 1;
                if !visited[ny][nx] && !grid[ny][nx] {
                    fringe.push((ny, nx));
                }
            }

            if sy < n_rows - 1 {
                let nx = sx;
                let ny = sy + 1;
                if !visited[ny][nx] && !grid[ny][nx] {
                    fringe.push((ny, nx));
                }
            }

            // show_grid(&visited);
            // dbg!(&fringe);
            // if fringe.len() > 10 {
            //     todo!();
            // }
        }
        // show_grid(&visited);

        let unfilled = visited
            .iter()
            .map(|line| line.iter().filter(|x| **x).count())
            .sum::<usize>();

        (n_rows * n_cols) - unfilled
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            62
        );
    }
}

mod part_2 {

    use std::iter;

    use itertools::Itertools;

    use super::part_1::*;

    pub fn solution(input: String) -> usize {
        use Dir::*;

        // let trenches = input
        // .lines()
        //     .map(|line| {
        //         let (dir, mag, hex) = line.split_whitespace().collect_tuple().unwrap();
        //         let dir = match dir {
        //             "U" => Up,
        //             "D" => Down,
        //             "L" => Left,
        //             "R" => Right,
        //             other => panic!("Unexpected direction {other}"),
        //         };
        //         let mag = mag.parse::<u32>().unwrap();
        //         (dir, mag)
        //     })
        //     .collect_vec();

        let trenches = input
            .lines()
            .map(|line| {
                let (_dir, _mag, hex) = line.split_whitespace().collect_tuple().unwrap();
                // dbg!(&hex[2..=7]);
                let (mag, dir) = hex[2..=7].split_at(5);
                let dir = match dir {
                    "3" => Up,
                    "1" => Down,
                    "2" => Left,
                    "0" => Right,
                    other => panic!("Unexpected direction {other}"),
                };
                let mag = usize::from_str_radix(mag, 16).unwrap();
                (dir, mag)
            })
            .collect_vec();

        // dbg!(&trenches);
        // todo!();

        let range_ud = trenches
            .iter()
            .filter_map(|(dir, mag)| {
                if dir == &Down {
                    Some(*mag as i32 + 1)
                } else if dir == &Up {
                    Some(-(*mag as i32 + 1))
                } else {
                    None
                }
            })
            .scan(0, |state, mag| {
                *state += mag;
                Some(*state)
            })
            .collect_vec();

        let range_lr = trenches
            .iter()
            .filter_map(|(dir, mag)| {
                if dir == &Right {
                    Some(*mag as i32 + 1)
                } else if dir == &Left {
                    Some(-(*mag as i32 + 1))
                } else {
                    None
                }
            })
            .scan(0, |state, mag| {
                *state += mag;
                Some(*state)
            })
            .collect_vec();

        // dbg!(&range_ud);
        // dbg!(&range_lr);

        let init_y = -(range_ud.iter().min().unwrap() - 1) as i64 + 1;
        let init_x = -(range_lr.iter().min().unwrap() - 1) as i64 + 1;

        // dbg!(init_x);
        // dbg!(init_y);

        let mut visited_start = false;
        let vertices = iter::once((init_x, init_y))
            .chain(
                trenches
                    .into_iter()
                    .scan((init_x, init_y), |(sx, sy), (dir, mag)| {
                        if *sx == init_x && *sy == init_y {
                            if visited_start {
                                return None;
                            } else {
                                visited_start = true;
                            }
                        }

                        match dir {
                            Up => *sy -= mag as i64,
                            Down => *sy += mag as i64,
                            Left => *sx -= mag as i64,
                            Right => *sx += mag as i64,
                        };

                        Some((*sx, *sy))
                    }),
            )
            .collect_vec();

        // dbg!(&vertices);

        let area = vertices
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|((y1, x1), (y2, x2))| (*x2 - *x1) * (*y2 + *y1))
            .sum::<i64>()
            / 2;

        // dbg!(interior);

        let boundary = vertices
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|((y1, x1), (y2, x2))| (*x2 - *x1).abs() + (*y2 - *y1).abs())
            .inspect(|_x| {
                // dbg!(_x);
            })
            .sum::<i64>();

        // dbg!(boundary);

        // Uses Pick's theorem and shoelace formula
        // https://11011110.github.io/blog/2021/04/17/picks-shoelaces.html

        // rearranged as follows
        // A = i + b/2 - 1
        // we want i + b
        // A + b/2 = i + b/2 +b/2 + 1
        // i + b = A + b/2 + 1

        // extra intuition as to why
        // the area is not directly the answer
        // the area would go through the "center"
        // of each line (aka an infinitely thin trench)
        // but since the trenches are "wide"
        // we add the half the the boundary
        // not contained in the area
        // but at each vertex we now either over
        // or under count a fraction of the corner
        // over a full polygon we will under-count
        // by a full square (360 degrees) so we add one

        (area + boundary / 2 + 1) as usize
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            952408144115
        );
    }
}
