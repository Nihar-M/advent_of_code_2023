pub const DAY_STR: &str = "day_16";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mirror {
        Empty,
        RisingMirror,
        FallingMirror,
        VerticalSplitter,
        HorizontalSplitter,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Beam {
        pub up: bool,
        pub down: bool,
        pub right: bool,
        pub left: bool,
    }

    impl From<char> for Mirror {
        fn from(value: char) -> Self {
            use Mirror::*;
            match value {
                '.' => Empty,
                '/' => RisingMirror,
                '\\' => FallingMirror,
                '|' => VerticalSplitter,
                '-' => HorizontalSplitter,
                other => panic!("Unexpected character found {other}"),
            }
        }
    }

    pub fn show_beams(beams: &Vec<Vec<Beam>>) {
        for row in beams {
            for b in row {
                print!(
                    "{}",
                    match (b.right, b.left, b.up, b.down) {
                        (true, true, true, true) => '⇹',
                        (true, true, true, false) => '⤉',
                        (true, true, false, true) => '⤈',
                        (true, true, false, false) => '↔',
                        (true, false, true, true) => '⇸',
                        (true, false, true, false) => '↗',
                        (true, false, false, true) => '↘',
                        (true, false, false, false) => '→',
                        (false, true, true, true) => '⇷',
                        (false, true, true, false) => '↖',
                        (false, true, false, true) => '↙',
                        (false, true, false, false) => '←',
                        (false, false, true, true) => '↕',
                        (false, false, true, false) => '↑',
                        (false, false, false, true) => '↓',
                        (false, false, false, false) => '.',
                    },
                );
            }
            println!();
        }
    }

    pub fn solution(input: String) -> usize {
        use Mirror::*;

        let grid = input
            .lines()
            .map(|row| row.chars().map(Mirror::from).collect_vec())
            .collect_vec();
        // dbg!(&grid);

        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        let mut beams: Vec<Vec<Beam>> = vec![
            vec![
                Beam {
                    up: false,
                    down: false,
                    right: false,
                    left: false
                };
                n_cols
            ];
            n_rows
        ];

        // beam enters (0,0) traveling to the RIGHT

        match &grid[0][0] {
            Empty => beams[0][0].right = true,
            RisingMirror => beams[0][0].up = true,
            FallingMirror => beams[0][0].down = true,
            VerticalSplitter => {
                beams[0][0].down = true;
                beams[0][0].up = true;
            }
            HorizontalSplitter => beams[0][0].right = true,
        }

        let mut heads: Vec<(usize, usize)> = vec![(0, 0)];

        // show_beams(&beams);

        while !heads.is_empty() {
            // dbg!(&heads);

            heads = heads
                .into_iter()
                .flat_map(|head| {
                    let y_i = head.0;
                    let x_i = head.1;
                    let beam = beams[y_i][x_i];

                    let mut new_heads = vec![];

                    if beam.right && x_i < n_cols - 1 {
                        let next_yi = y_i;
                        let next_xi = x_i + 1;

                        let next_beam_cell = &mut beams[next_yi][next_xi];

                        match grid[next_yi][next_xi] {
                            Empty => {
                                if !next_beam_cell.right {
                                    next_beam_cell.right = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            RisingMirror => {
                                if !next_beam_cell.up {
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            FallingMirror => {
                                if !next_beam_cell.down {
                                    next_beam_cell.down = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            VerticalSplitter => {
                                if !next_beam_cell.down || !next_beam_cell.up {
                                    next_beam_cell.down = true;
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            HorizontalSplitter => {
                                if !next_beam_cell.right {
                                    next_beam_cell.right = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                        }
                    }
                    if beam.left && x_i > 0 {
                        let next_yi = y_i;
                        let next_xi = x_i - 1;

                        let next_beam_cell = &mut beams[next_yi][next_xi];

                        match grid[next_yi][next_xi] {
                            Empty => {
                                if !next_beam_cell.left {
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            RisingMirror => {
                                if !next_beam_cell.down {
                                    next_beam_cell.down = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            FallingMirror => {
                                if !next_beam_cell.up {
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            VerticalSplitter => {
                                if !next_beam_cell.down || !next_beam_cell.up {
                                    next_beam_cell.down = true;
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            HorizontalSplitter => {
                                if !next_beam_cell.left {
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                        }
                    }

                    if beam.up && y_i > 0 {
                        let next_yi = y_i - 1;
                        let next_xi = x_i;

                        let next_beam_cell = &mut beams[next_yi][next_xi];

                        match grid[next_yi][next_xi] {
                            Empty => {
                                if !next_beam_cell.up {
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            RisingMirror => {
                                if !next_beam_cell.right {
                                    next_beam_cell.right = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            FallingMirror => {
                                if !next_beam_cell.left {
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            VerticalSplitter => {
                                if !next_beam_cell.up {
                                    next_beam_cell.up = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            HorizontalSplitter => {
                                if !next_beam_cell.right || !next_beam_cell.left {
                                    next_beam_cell.right = true;
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                        }
                    }

                    if beam.down && y_i < n_rows - 1 {
                        let next_yi = y_i + 1;
                        let next_xi = x_i;

                        let next_beam_cell = &mut beams[next_yi][next_xi];

                        match grid[next_yi][next_xi] {
                            Empty => {
                                if !next_beam_cell.down {
                                    next_beam_cell.down = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            RisingMirror => {
                                if !next_beam_cell.left {
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            FallingMirror => {
                                if !next_beam_cell.right {
                                    next_beam_cell.right = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            VerticalSplitter => {
                                if !next_beam_cell.down {
                                    next_beam_cell.down = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                            HorizontalSplitter => {
                                if !next_beam_cell.right || !next_beam_cell.left {
                                    next_beam_cell.right = true;
                                    next_beam_cell.left = true;
                                    new_heads.push((next_yi, next_xi));
                                }
                            }
                        }
                    }

                    // dbg!(beam);
                    new_heads
                })
                .collect_vec();
            // show_beams(&beams);
        }

        beams
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|c| c.down || c.up || c.left || c.right)
                    .count()
            })
            .sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            46
        );
    }
}

mod part_2 {

    use itertools::Itertools;

    use super::part_1::*;

    pub fn solution(input: String) -> usize {
        use Mirror::*;

        let grid = input
            .lines()
            .map(|row| row.chars().map(Mirror::from).collect_vec())
            .collect_vec();
        // dbg!(&grid);

        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        let starting_points_left = (0..n_rows)
            .map(|y_i| {
                // beam enters (0,0) traveling to the LEFT
                let start_y = y_i;
                let start_x = n_cols - 1;
                let cell_state = &grid[start_y][start_x];
                // dbg!(cell_state);
                (
                    (start_y, start_x),
                    Beam {
                        up: *cell_state == FallingMirror || *cell_state == VerticalSplitter,
                        down: *cell_state == RisingMirror || *cell_state == VerticalSplitter,
                        right: false,
                        left: *cell_state == Empty || *cell_state == HorizontalSplitter,
                    },
                )
            })
            .collect_vec();
        // dbg!(&starting_points_left);

        let starting_points_up = (0..n_cols)
            .map(|x_i| {
                // beam enters (0,0) traveling UP
                let start_y = n_rows - 1;
                let start_x = x_i;
                let cell_state = &grid[start_y][start_x];
                // dbg!(cell_state);
                (
                    (start_y, start_x),
                    Beam {
                        up: *cell_state == Empty || *cell_state == VerticalSplitter,
                        down: false,
                        right: *cell_state == RisingMirror || *cell_state == HorizontalSplitter,
                        left: *cell_state == FallingMirror || *cell_state == HorizontalSplitter,
                    },
                )
            })
            .collect_vec();

        // dbg!(&starting_points_up);

        let starting_points_right = (0..n_rows)
            .map(|y_i| {
                // beam enters (0,0) traveling to the RIGHT
                let start_y = y_i;
                let start_x = 0;
                let cell_state = &grid[start_y][start_x];
                // dbg!(cell_state);
                (
                    (start_y, start_x),
                    Beam {
                        up: *cell_state == RisingMirror || *cell_state == VerticalSplitter,
                        down: *cell_state == FallingMirror || *cell_state == VerticalSplitter,
                        right: *cell_state == Empty || *cell_state == HorizontalSplitter,
                        left: false,
                    },
                )
            })
            .collect_vec();
        // dbg!(&starting_points_right);

        let starting_points_down = (0..n_cols)
            .map(|x_i| {
                // beam enters (0,0) traveling DOWN
                let start_y = 0;
                let start_x = x_i;
                let cell_state = &grid[start_y][start_x];
                // dbg!(cell_state);
                (
                    (start_y, start_x),
                    Beam {
                        up: false,
                        down: *cell_state == Empty || *cell_state == VerticalSplitter,
                        right: *cell_state == FallingMirror || *cell_state == HorizontalSplitter,
                        left: *cell_state == RisingMirror || *cell_state == HorizontalSplitter,
                    },
                )
            })
            .collect_vec();

        // dbg!(&starting_points_down);

        let starting_points = starting_points_left
            .into_iter()
            .chain(starting_points_up)
            .chain(starting_points_right)
            .chain(starting_points_down)
            .collect_vec();

        starting_points
            .into_iter()
            .map(|((sy, sx), beam)| {
                let mut heads: Vec<(usize, usize)> = vec![(sy, sx)];
                let mut beams: Vec<Vec<Beam>> = vec![
                    vec![
                        Beam {
                            up: false,
                            down: false,
                            right: false,
                            left: false
                        };
                        n_cols
                    ];
                    n_rows
                ];

                beams[sy][sx] = beam;

                while !heads.is_empty() {
                    // dbg!(&heads);

                    heads = heads
                        .into_iter()
                        .flat_map(|head| {
                            let y_i = head.0;
                            let x_i = head.1;
                            let beam = beams[y_i][x_i];

                            let mut new_heads = vec![];

                            if beam.right && x_i < n_cols - 1 {
                                let next_yi = y_i;
                                let next_xi = x_i + 1;

                                let next_beam_cell = &mut beams[next_yi][next_xi];

                                match grid[next_yi][next_xi] {
                                    Empty => {
                                        if !next_beam_cell.right {
                                            next_beam_cell.right = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    RisingMirror => {
                                        if !next_beam_cell.up {
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    FallingMirror => {
                                        if !next_beam_cell.down {
                                            next_beam_cell.down = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    VerticalSplitter => {
                                        if !next_beam_cell.down || !next_beam_cell.up {
                                            next_beam_cell.down = true;
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    HorizontalSplitter => {
                                        if !next_beam_cell.right {
                                            next_beam_cell.right = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                }
                            }
                            if beam.left && x_i > 0 {
                                let next_yi = y_i;
                                let next_xi = x_i - 1;

                                let next_beam_cell = &mut beams[next_yi][next_xi];

                                match grid[next_yi][next_xi] {
                                    Empty => {
                                        if !next_beam_cell.left {
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    RisingMirror => {
                                        if !next_beam_cell.down {
                                            next_beam_cell.down = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    FallingMirror => {
                                        if !next_beam_cell.up {
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    VerticalSplitter => {
                                        if !next_beam_cell.down || !next_beam_cell.up {
                                            next_beam_cell.down = true;
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    HorizontalSplitter => {
                                        if !next_beam_cell.left {
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                }
                            }

                            if beam.up && y_i > 0 {
                                let next_yi = y_i - 1;
                                let next_xi = x_i;

                                let next_beam_cell = &mut beams[next_yi][next_xi];

                                match grid[next_yi][next_xi] {
                                    Empty => {
                                        if !next_beam_cell.up {
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    RisingMirror => {
                                        if !next_beam_cell.right {
                                            next_beam_cell.right = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    FallingMirror => {
                                        if !next_beam_cell.left {
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    VerticalSplitter => {
                                        if !next_beam_cell.up {
                                            next_beam_cell.up = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    HorizontalSplitter => {
                                        if !next_beam_cell.right || !next_beam_cell.left {
                                            next_beam_cell.right = true;
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                }
                            }

                            if beam.down && y_i < n_rows - 1 {
                                let next_yi = y_i + 1;
                                let next_xi = x_i;

                                let next_beam_cell = &mut beams[next_yi][next_xi];

                                match grid[next_yi][next_xi] {
                                    Empty => {
                                        if !next_beam_cell.down {
                                            next_beam_cell.down = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    RisingMirror => {
                                        if !next_beam_cell.left {
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    FallingMirror => {
                                        if !next_beam_cell.right {
                                            next_beam_cell.right = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    VerticalSplitter => {
                                        if !next_beam_cell.down {
                                            next_beam_cell.down = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                    HorizontalSplitter => {
                                        if !next_beam_cell.right || !next_beam_cell.left {
                                            next_beam_cell.right = true;
                                            next_beam_cell.left = true;
                                            new_heads.push((next_yi, next_xi));
                                        }
                                    }
                                }
                            }

                            // dbg!(beam);
                            new_heads
                        })
                        .collect_vec();
                    // show_beams(&beams);
                }

                beams
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|c| c.down || c.up || c.left || c.right)
                            .count()
                    })
                    .sum::<usize>()
            })
            .max()
            .unwrap()
    }
    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            51
        );
    }
}
