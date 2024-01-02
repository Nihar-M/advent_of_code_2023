pub const DAY_STR: &str = "day_10";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::fmt::{Debug, Write};

    #[derive(Debug, Clone, Copy)]
    pub struct GridPos {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Dir {
        North,
        South,
        West,
        East,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Pipe {
        Vertical,
        Horizontal,
        TurnNE,
        TurnNW,
        TurnSE,
        TurnSW,
        Ground,
        Start,
    }

    pub struct Grid {
        pub vec2d: Vec<Vec<Pipe>>,
        pub start: GridPos,
    }

    impl Dir {
        fn offset(&self) -> GridPos {
            use Dir::*;
            match &self {
                North => GridPos { x: 0, y: -1 },
                South => GridPos { x: 0, y: 1 },
                West => GridPos { x: -1, y: 0 },
                East => GridPos { x: 1, y: 0 },
            }
        }

        fn reverse(self) -> Dir {
            match self {
                Dir::North => Dir::South,
                Dir::South => Dir::North,
                Dir::West => Dir::East,
                Dir::East => Dir::West,
            }
        }
    }

    impl GridPos {
        fn moved(self, dir: Dir) -> GridPos {
            let offset = dir.offset();
            GridPos {
                x: self.x + offset.x,
                y: self.y + offset.y,
            }
        }

        fn try_as_usize_tuple(self) -> Option<(usize, usize)> {
            Some((self.x.try_into().ok()?, self.y.try_into().ok()?))
        }
    }

    impl Pipe {
        fn connections(self) -> Option<(Dir, Dir)> {
            use Dir::*;
            use Pipe::*;
            match self {
                Vertical => Some((North, South)),
                Horizontal => Some((West, East)),
                TurnNE => Some((North, East)),
                TurnNW => Some((North, West)),
                TurnSE => Some((South, East)),
                TurnSW => Some((South, West)),
                Ground => None,
                Start => None,
            }
        }

        fn get_other_dir(self, dir: Dir) -> Dir {
            match dir {
                Dir::North => match self {
                    Pipe::Vertical => Dir::South,
                    Pipe::TurnNE => Dir::East,
                    Pipe::TurnNW => Dir::West,
                    _ => panic!(),
                },
                Dir::South => match self {
                    Pipe::Vertical => Dir::North,
                    Pipe::TurnSE => Dir::East,
                    Pipe::TurnSW => Dir::West,
                    _ => panic!(),
                },
                Dir::West => match self {
                    Pipe::Horizontal => Dir::East,
                    Pipe::TurnNW => Dir::North,
                    Pipe::TurnSW => Dir::South,
                    _ => panic!(),
                },
                Dir::East => match self {
                    Pipe::Horizontal => Dir::West,
                    Pipe::TurnNE => Dir::North,
                    Pipe::TurnSE => Dir::South,
                    _ => panic!(),
                },
            }
        }

        // dir is the direction they are entering the pipe from
        // this is opposite to the direction relative to the pipe
        fn get_next_dir(self, dir: Dir) -> Dir {
            self.get_other_dir(dir.reverse())
        }
    }

    impl From<char> for Pipe {
        fn from(value: char) -> Self {
            use Pipe::*;
            match value {
                'S' => Start,
                '.' => Ground,
                '-' => Horizontal,
                '|' => Vertical,
                'L' => TurnNE,
                'J' => TurnNW,
                'F' => TurnSE,
                '7' => TurnSW,
                x => panic!("Invalid char `{x}` found. Cannot parse into `Pipe`"),
            }
        }
    }

    impl From<Pipe> for char {
        fn from(value: Pipe) -> Self {
            use Pipe::*;
            match value {
                Start => 'S',
                Ground => '.',
                Horizontal => '-',
                Vertical => '|',
                TurnNE => 'L',
                TurnNW => 'J',
                TurnSE => 'F',
                TurnSW => '7',
            }
        }
    }

    impl Grid {
        pub fn new(mut vec2d: Vec<Vec<Pipe>>) -> Grid {
            let start = vec2d
                .iter()
                .enumerate()
                .find_map(|(y, row)| {
                    row.iter().enumerate().find_map(|(x, pipe)| match pipe {
                        Pipe::Start => Some(GridPos {
                            x: x as i32,
                            y: y as i32,
                        }),
                        _ => None,
                    })
                })
                .unwrap();
            // dbg!(start);

            let north = start
                .moved(Dir::North)
                .try_as_usize_tuple()
                .map(|(ix, iy)| vec2d[iy][ix]);

            let south = start
                .moved(Dir::South)
                .try_as_usize_tuple()
                .map(|(ix, iy)| vec2d[iy][ix]);

            let west = start
                .moved(Dir::West)
                .try_as_usize_tuple()
                .map(|(ix, iy)| vec2d[iy][ix]);

            let east = start
                .moved(Dir::East)
                .try_as_usize_tuple()
                .map(|(ix, iy)| vec2d[iy][ix]);

            // valid connectors
            // north : Pipe::Vertical | Pipe::TurnSE | Pipe::TurnSW
            // south : Pipe::Vertical | Pipe::TurnNE | Pipe::TurnNW
            // west  : Pipe::Horizontal | Pipe::TurnNE | Pipe::TurnSE
            // east  : Pipe::Horizontal | Pipe::TurnNW | Pipe::TurnSW

            let start_pipe = match (north, south, west, east) {
                (
                    _,
                    _,
                    Some(Pipe::Horizontal | Pipe::TurnNE | Pipe::TurnSE),
                    Some(Pipe::Horizontal | Pipe::TurnNW | Pipe::TurnSW),
                ) => Pipe::Horizontal,
                (
                    _,
                    Some(Pipe::Vertical | Pipe::TurnNE | Pipe::TurnNW),
                    _,
                    Some(Pipe::Horizontal | Pipe::TurnNW | Pipe::TurnSW),
                ) => Pipe::TurnSE,
                (
                    _,
                    Some(Pipe::Vertical | Pipe::TurnNE | Pipe::TurnNW),
                    Some(Pipe::Horizontal | Pipe::TurnNE | Pipe::TurnSE),
                    _,
                ) => Pipe::TurnSW,
                (
                    Some(Pipe::Vertical | Pipe::TurnSE | Pipe::TurnSW),
                    _,
                    _,
                    Some(Pipe::Horizontal | Pipe::TurnNW | Pipe::TurnSW),
                ) => Pipe::TurnNE,
                (
                    Some(Pipe::Vertical | Pipe::TurnSE | Pipe::TurnSW),
                    _,
                    Some(Pipe::Horizontal | Pipe::TurnNE | Pipe::TurnSE),
                    _,
                ) => Pipe::TurnNW,
                (
                    Some(Pipe::Vertical | Pipe::TurnSE | Pipe::TurnSW),
                    Some(Pipe::Vertical | Pipe::TurnNE | Pipe::TurnNW),
                    _,
                    _,
                ) => Pipe::Vertical,
                unknown => panic!("Unexpected setup found {:?}", unknown),
            };

            // dbg!(start_pipe);

            let (sx, sy) = start.try_as_usize_tuple().unwrap();

            vec2d[sy][sx] = start_pipe;

            Grid { vec2d, start }
        }
    }

    impl Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Grid { ")?;
            for row in &self.vec2d {
                f.write_str("\n\t")?;
                for pipe in row {
                    f.write_char(char::from(*pipe))?;
                }
            }
            f.write_fmt(format_args!(
                "\n}} start: {{ x : {} , y : {} }}\n",
                self.start.x, self.start.y
            ))
        }
    }

    impl Grid {
        pub fn get_pipe(&self, grid_pos: GridPos) -> Pipe {
            grid_pos
                .try_as_usize_tuple()
                .map(|(ix, iy)| self.vec2d[iy][ix])
                .expect("GridPos was invalid")
        }

        pub fn get_pipe_ref_mut(&mut self, grid_pos: GridPos) -> &mut Pipe {
            grid_pos
                .try_as_usize_tuple()
                .map(|(ix, iy)| &mut self.vec2d[iy][ix])
                .expect("GridPos was invalid")
        }

        pub fn walk(&self, grid_pos: GridPos, last_dir: Dir) -> (Dir, GridPos) {
            let move_dir = self.get_pipe(grid_pos).get_next_dir(last_dir);
            (move_dir, grid_pos.moved(move_dir))
        }
    }

    pub fn solution(input: String) -> usize {
        let pipes = input
            .lines()
            .map(|line| line.chars().map(Pipe::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let grid = Grid::new(pipes);

        // dbg!(&grid);

        let starting_blocked_dir = match grid.get_pipe(grid.start) {
            Pipe::Vertical => Dir::North,
            Pipe::Horizontal => Dir::East,
            Pipe::TurnNE => Dir::South,
            Pipe::TurnNW => Dir::South,
            Pipe::TurnSE => Dir::North,
            Pipe::TurnSW => Dir::North,
            Pipe::Ground => panic!(),
            Pipe::Start => panic!(),
        };

        // dbg!(starting_blocked_dir, grid.start);
        let (mut last_move, mut current_pos) = (starting_blocked_dir, grid.start);
        // let (mut last_move, mut starting_pos) = grid.walk(grid.start, starting_blocked_dir);

        let path = std::iter::from_fn(move || {
            // dbg!(last_move, current_pos, grid.get_pipe(current_pos));
            (last_move, current_pos) = grid.walk(current_pos, last_move);
            // dbg!(last_move, current_pos, grid.get_pipe(current_pos));

            // todo!();
            if current_pos.x == grid.start.x && current_pos.y == grid.start.y {
                None
            } else {
                Some(current_pos)
            }
        })
        .enumerate()
        .inspect(|(iter, _pos)| {
            assert!(*iter < 100000);
            // dbg!(pos);
        })
        .collect::<Vec<_>>();

        // dbg!(&path);

        let cycle_length = path.len() + 1;

        // dbg!(cycle_length);
        // dbg!(furthest);

        cycle_length.checked_div(2).unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            4
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample2.txt", super::DAY_STR))
                    .unwrap()
            ),
            8
        );
    }
}

mod part_2 {

    use super::part_1::*;

    #[derive(Debug, Clone, Copy)]
    enum State {
        Inside,
        Outside,
        InsideEdgeWithNE,
        InsideEdgeWithSE,
        OutsideEdgeWithNE,
        OutsideEdgeWithSE,
    }

    pub fn solution(input: String) -> usize {
        let pipes = input
            .lines()
            .map(|line| line.chars().map(Pipe::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut grid = Grid::new(pipes);

        // dbg!(&grid);

        let starting_blocked_dir = match grid.get_pipe(grid.start) {
            Pipe::Vertical => Dir::North,
            Pipe::Horizontal => Dir::East,
            Pipe::TurnNE => Dir::South,
            Pipe::TurnNW => Dir::South,
            Pipe::TurnSE => Dir::North,
            Pipe::TurnSW => Dir::North,
            Pipe::Ground => panic!(),
            Pipe::Start => panic!(),
        };

        // dbg!(starting_blocked_dir, grid.start);
        let (mut last_move, mut current_pos) = (starting_blocked_dir, grid.start);
        // let (mut last_move, mut starting_pos) = grid.walk(grid.start, starting_blocked_dir);

        let path = std::iter::from_fn(|| {
            // dbg!(last_move, current_pos, grid.get_pipe(current_pos));
            (last_move, current_pos) = grid.walk(current_pos, last_move);
            // dbg!(last_move, current_pos, grid.get_pipe(current_pos));

            // todo!();
            if current_pos.x == grid.start.x && current_pos.y == grid.start.y {
                None
            } else {
                Some(current_pos)
            }
        })
        .enumerate()
        .inspect(|(iter, _pos)| {
            assert!(*iter < 100000);
            // dbg!(pos);
        })
        .collect::<Vec<_>>();

        let mut path = path
            .iter()
            .map(|p| (p.0, p.1, grid.get_pipe(p.1)))
            .collect::<Vec<_>>();

        path.push((path.len() + 1, grid.start, grid.get_pipe(grid.start)));

        let path = path; // path doesn't really need to be mut

        // clear the grid
        grid.vec2d
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|pipe| *pipe = Pipe::Ground));

        // restore the path
        path.iter().for_each(|(_idx, pos, pipe)| {
            *grid.get_pipe_ref_mut(*pos) = *pipe;
        });

        // dbg!(&grid);

        grid.vec2d.iter_mut().for_each(|row| {
            let mut state = State::Outside;
            row.iter_mut().for_each(|pipe| {
                // dbg!(state);
                match state {
                    State::Inside => {
                        state = match pipe {
                            Pipe::Vertical => State::Outside,
                            Pipe::Horizontal => panic!(),
                            Pipe::TurnNE => State::InsideEdgeWithNE,
                            Pipe::TurnNW => panic!(),
                            Pipe::TurnSE => State::InsideEdgeWithSE,
                            Pipe::TurnSW => panic!(),
                            Pipe::Ground => {
                                *pipe = Pipe::Start;
                                State::Inside
                            }
                            Pipe::Start => panic!(),
                        }
                    }
                    State::Outside => {
                        state = match pipe {
                            Pipe::Vertical => State::Inside,
                            Pipe::Horizontal => panic!(),
                            Pipe::TurnNE => State::OutsideEdgeWithNE,
                            Pipe::TurnNW => todo!(),
                            Pipe::TurnSE => State::OutsideEdgeWithSE,
                            Pipe::TurnSW => panic!(),
                            Pipe::Ground => {
                                *pipe = Pipe::Ground;
                                State::Outside
                            }
                            Pipe::Start => panic!(),
                        }
                    }
                    State::OutsideEdgeWithNE => {
                        state = match pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => State::OutsideEdgeWithNE,
                            Pipe::TurnNE => panic!(),
                            Pipe::TurnNW => State::Outside,
                            Pipe::TurnSE => panic!(),
                            Pipe::TurnSW => State::Inside,
                            Pipe::Ground => panic!(),
                            Pipe::Start => panic!(),
                        }
                    }
                    State::OutsideEdgeWithSE => {
                        state = match pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => State::OutsideEdgeWithSE,
                            Pipe::TurnNE => panic!(),
                            Pipe::TurnNW => State::Inside,
                            Pipe::TurnSE => panic!(),
                            Pipe::TurnSW => State::Outside,
                            Pipe::Ground => panic!(),
                            Pipe::Start => panic!(),
                        }
                    }
                    State::InsideEdgeWithNE => {
                        state = match pipe {
                            Pipe::Vertical => panic!(),
                            Pipe::Horizontal => State::InsideEdgeWithNE,
                            Pipe::TurnNE => panic!(),
                            Pipe::TurnNW => State::Inside,
                            Pipe::TurnSE => panic!(),
                            Pipe::TurnSW => State::Outside,
                            Pipe::Ground => panic!(),
                            Pipe::Start => panic!(),
                        }
                    }
                    State::InsideEdgeWithSE => {
                        state = match pipe {
                            Pipe::Vertical => todo!(),
                            Pipe::Horizontal => State::InsideEdgeWithSE,
                            Pipe::TurnNE => todo!(),
                            Pipe::TurnNW => State::Outside,
                            Pipe::TurnSE => todo!(),
                            Pipe::TurnSW => State::Inside,
                            Pipe::Ground => todo!(),
                            Pipe::Start => todo!(),
                        }
                    }
                }
                // dbg!(state);
            });
        });

        // dbg!(&grid);

        let x = grid
            .vec2d
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pipe| match pipe {
                        Pipe::Start => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();

        // dbg!(x);
        x
        // todo!()
    }

    // pub fn solution(input: String) -> usize {
    //     todo!()
    // }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            1
        );
    }

    #[test]
    fn sample2() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample2.txt", super::DAY_STR))
                    .unwrap()
            ),
            1
        );
    }

    #[test]
    fn sample3() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample3.txt", super::DAY_STR))
                    .unwrap()
            ),
            4
        );
    }

    #[test]
    fn sample4() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample4.txt", super::DAY_STR))
                    .unwrap()
            ),
            4
        );
    }

    #[test]
    fn sample5() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample5.txt", super::DAY_STR))
                    .unwrap()
            ),
            8
        );
    }

    #[test]
    fn sample6() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample6.txt", super::DAY_STR))
                    .unwrap()
            ),
            10
        );
    }
}
