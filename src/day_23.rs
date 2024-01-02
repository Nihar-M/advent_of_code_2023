pub const DAY_STR: &str = "day_23";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use itertools::Itertools;

    use petgraph::algo::all_simple_paths;
    // use petgraph::dot::Dot;
    use petgraph::prelude::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Tile {
        Wall,
        Ground,
        USlope,
        DSlope,
        LSlope,
        RSlope,
    }

    type Grid = Vec<Vec<Tile>>;

    pub fn show_grid(grid: &Grid) {
        for row in grid {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Wall => '#',
                        Tile::Ground => '.',
                        Tile::USlope => '^',
                        Tile::DSlope => 'v',
                        Tile::LSlope => '<',
                        Tile::RSlope => '>',
                    },
                )
            }
            println!();
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Vertex {
        pub y: usize,
        pub x: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Edge {
        pub start: Vertex,
        pub end: Vertex,
        pub length: usize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Dir {
        U,
        D,
        L,
        R,
    }

    pub fn solution(input: String) -> usize {
        use Tile::*;
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Wall,
                        '.' => Ground,
                        '^' => USlope,
                        'v' => DSlope,
                        '<' => LSlope,
                        '>' => RSlope,
                        other => panic!("Unexpected tile {other}"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        // show_grid(&grid);

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        let starting_vertex = Vertex {
            y: 0,
            x: (1..n_rows).find(|i| grid[0][*i] == Ground).unwrap(),
        };

        let ending_vertex = Vertex {
            y: n_rows - 1,
            x: (1..n_rows)
                .find(|i| grid[n_rows - 1][*i] == Ground)
                .unwrap(),
        };

        let mut vertices = (1..n_rows - 1)
            .cartesian_product(1..n_cols - 1)
            .filter_map(|(y, x)| {
                let tile = grid[y][x];
                if tile != Wall {
                    let u = grid[y - 1][x];
                    let d = grid[y + 1][x];
                    let l = grid[y][x - 1];
                    let r = grid[y][x + 1];
                    let non_walls = [u, d, l, r]
                        .into_iter()
                        .filter(|t| *t != Wall)
                        .collect_vec();
                    if non_walls.len() > 2 {
                        // dbg!(tile);
                        // dbg!(y);
                        // dbg!(x);
                        return Some(Vertex { y, x });
                    }
                }
                None
            })
            .collect_vec();

        vertices.push(starting_vertex);
        vertices.push(ending_vertex);

        // dbg!(&vertices);

        let edges = vertices
            .iter()
            .flat_map(|vertex| {
                // dbg!(vertex);

                let starting_dirs = if vertex == &starting_vertex {
                    vec![Dir::D]
                } else if vertex == &ending_vertex {
                    vec![Dir::U]
                } else {
                    vec![Dir::U, Dir::D, Dir::L, Dir::R]
                        .into_iter()
                        .filter(|dir| match dir {
                            Dir::U => grid[vertex.y - 1][vertex.x] != Tile::Wall,
                            Dir::D => grid[vertex.y + 1][vertex.x] != Tile::Wall,
                            Dir::L => grid[vertex.y][vertex.x - 1] != Tile::Wall,
                            Dir::R => grid[vertex.y][vertex.x + 1] != Tile::Wall,
                        })
                        .collect_vec()
                };

                // dbg!(&starting_dirs);

                // dbg!(&adjacent_edges);
                // vec![0]
                starting_dirs
                    .into_iter()
                    .filter_map(|start_dir| {
                        let (mut y, mut x) = match start_dir {
                            Dir::U => (vertex.y - 1, vertex.x),
                            Dir::D => (vertex.y + 1, vertex.x),
                            Dir::L => (vertex.y, vertex.x - 1),
                            Dir::R => (vertex.y, vertex.x + 1),
                        };
                        // dbg!(start_dir);

                        let starting_tile = grid[y][x];
                        if match start_dir {
                            Dir::U => starting_tile == DSlope,
                            Dir::D => starting_tile == USlope,
                            Dir::L => starting_tile == RSlope,
                            Dir::R => starting_tile == LSlope,
                        } {
                            return None;
                        }

                        let mut last_dir = start_dir;

                        let mut num_steps = 0;
                        loop {
                            num_steps += 1;
                            // dbg!(last_dir);
                            let mut non_wall_tiles = 0;

                            let open_squares = vec![Dir::U, Dir::D, Dir::L, Dir::R]
                                .into_iter()
                                .filter(|dir| {
                                    // removes direction that we came from
                                    match last_dir {
                                        Dir::U => *dir != Dir::D,
                                        Dir::D => *dir != Dir::U,
                                        Dir::L => *dir != Dir::R,
                                        Dir::R => *dir != Dir::L,
                                    }
                                })
                                .filter_map(|dir| {
                                    let (n_y, n_x) = match dir {
                                        Dir::U => (y - 1, x),
                                        Dir::D => (y + 1, x),
                                        Dir::L => (y, x - 1),
                                        Dir::R => (y, x + 1),
                                    };
                                    let n_tile = grid[n_y][n_x];
                                    // dbg!(dir);
                                    // dbg!(n_tile);
                                    if n_tile == Wall {
                                        None
                                    } else if dir == Dir::U && n_tile == DSlope
                                        || dir == Dir::D && n_tile == USlope
                                        || dir == Dir::L && n_tile == RSlope
                                        || dir == Dir::R && n_tile == LSlope
                                    {
                                        non_wall_tiles += 1;
                                        None
                                    } else {
                                        non_wall_tiles += 1;
                                        Some(((n_y, n_x), dir))
                                    }

                                    // todo!()
                                })
                                .collect_vec();

                            // dbg!(non_wall_tiles);
                            // dbg!(&open_squares);

                            if non_wall_tiles == 0 || open_squares.is_empty() {
                                break None;
                            } else if non_wall_tiles > 1 {
                                // dbg!(y);
                                // dbg!(x);
                                break Some(Edge {
                                    start: *vertex,
                                    end: Vertex { y, x },
                                    length: num_steps,
                                });
                            }

                            ((y, x), last_dir) = open_squares[0];

                            if y == 0 || y == n_rows - 1 {
                                break Some(Edge {
                                    start: *vertex,
                                    end: Vertex { y, x },
                                    length: num_steps + 1,
                                });
                            }
                            // todo!();
                        }

                        // todo!();
                        // 0
                    })
                    .collect_vec()
                // todo!();
            })
            .collect_vec();

        // dbg!(&edges);

        // let num_vertices = vertices.len();
        // dbg!(num_vertices);
        // let num_edges = edges.len();
        // dbg!(num_edges);

        let graph = DiGraphMap::<Vertex, usize>::from_edges(
            edges.iter().map(|e| (e.start, e.end, e.length)),
        );

        // let g = Dot::with_config(&graph, &[]);
        // dbg!(g);

        let paths: Vec<Vec<_>> =
            all_simple_paths(&graph, starting_vertex, ending_vertex, 0, None).collect_vec();

        let path_lengths = paths
            .iter()
            .map(|path| {
                path.iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| graph.edge_weight(*a, *b).unwrap())
                    .sum::<usize>()
            })
            .collect_vec();

        // dbg!(&path_lengths);

        *path_lengths.iter().max().unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            94
        );
    }
}

mod part_2 {

    use itertools::Itertools;

    use petgraph::algo::all_simple_paths;
    // use petgraph::dot::Dot;
    use petgraph::prelude::*;

    use super::part_1::*;

    pub fn solution(input: String) -> usize {
        use Tile::*;
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '#' => Wall,
                        '.' => Ground,
                        '^' => USlope,
                        'v' => DSlope,
                        '<' => LSlope,
                        '>' => RSlope,
                        other => panic!("Unexpected tile {other}"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        // show_grid(&grid);

        let n_rows = grid.len();
        let n_cols = grid[0].len();

        let starting_vertex = Vertex {
            y: 0,
            x: (1..n_rows).find(|i| grid[0][*i] == Ground).unwrap(),
        };

        let ending_vertex = Vertex {
            y: n_rows - 1,
            x: (1..n_rows)
                .find(|i| grid[n_rows - 1][*i] == Ground)
                .unwrap(),
        };

        let mut vertices = (1..n_rows - 1)
            .cartesian_product(1..n_cols - 1)
            .filter_map(|(y, x)| {
                let tile = grid[y][x];
                if tile != Wall {
                    let u = grid[y - 1][x];
                    let d = grid[y + 1][x];
                    let l = grid[y][x - 1];
                    let r = grid[y][x + 1];
                    let non_walls = [u, d, l, r]
                        .into_iter()
                        .filter(|t| *t != Wall)
                        .collect_vec();
                    if non_walls.len() > 2 {
                        // dbg!(tile);
                        // dbg!(y);
                        // dbg!(x);
                        return Some(Vertex { y, x });
                    }
                }
                None
            })
            .collect_vec();

        vertices.push(starting_vertex);
        vertices.push(ending_vertex);

        // dbg!(&vertices);

        let edges = vertices
            .iter()
            .flat_map(|vertex| {
                // dbg!(vertex);

                let starting_dirs = if vertex == &starting_vertex {
                    vec![Dir::D]
                } else if vertex == &ending_vertex {
                    vec![Dir::U]
                } else {
                    vec![Dir::U, Dir::D, Dir::L, Dir::R]
                        .into_iter()
                        .filter(|dir| match dir {
                            Dir::U => grid[vertex.y - 1][vertex.x] != Tile::Wall,
                            Dir::D => grid[vertex.y + 1][vertex.x] != Tile::Wall,
                            Dir::L => grid[vertex.y][vertex.x - 1] != Tile::Wall,
                            Dir::R => grid[vertex.y][vertex.x + 1] != Tile::Wall,
                        })
                        .collect_vec()
                };

                // dbg!(&starting_dirs);
                starting_dirs
                    .into_iter()
                    .filter_map(|start_dir| {
                        let (mut y, mut x) = match start_dir {
                            Dir::U => (vertex.y - 1, vertex.x),
                            Dir::D => (vertex.y + 1, vertex.x),
                            Dir::L => (vertex.y, vertex.x - 1),
                            Dir::R => (vertex.y, vertex.x + 1),
                        };
                        // dbg!(start_dir);

                        // let starting_tile = grid[y][x];
                        // if match start_dir {
                        //     Dir::U => starting_tile == DSlope,
                        //     Dir::D => starting_tile == USlope,
                        //     Dir::L => starting_tile == RSlope,
                        //     Dir::R => starting_tile == LSlope,
                        // } {
                        //     return None;
                        // }

                        let mut last_dir = start_dir;

                        let mut num_steps = 0;
                        loop {
                            num_steps += 1;
                            // dbg!(last_dir);
                            let mut non_wall_tiles = 0;

                            let open_squares = vec![Dir::U, Dir::D, Dir::L, Dir::R]
                                .into_iter()
                                .filter(|dir| {
                                    // removes direction that we came from
                                    match last_dir {
                                        Dir::U => *dir != Dir::D,
                                        Dir::D => *dir != Dir::U,
                                        Dir::L => *dir != Dir::R,
                                        Dir::R => *dir != Dir::L,
                                    }
                                })
                                .filter_map(|dir| {
                                    let (n_y, n_x) = match dir {
                                        Dir::U => (y - 1, x),
                                        Dir::D => (y + 1, x),
                                        Dir::L => (y, x - 1),
                                        Dir::R => (y, x + 1),
                                    };
                                    let n_tile = grid[n_y][n_x];
                                    // dbg!(dir);
                                    // dbg!(n_tile);
                                    if n_tile == Wall {
                                        None
                                    } else {
                                        non_wall_tiles += 1;
                                        Some(((n_y, n_x), dir))
                                    }
                                    /* else if dir == Dir::U && n_tile == DSlope
                                        || dir == Dir::D && n_tile == USlope
                                        || dir == Dir::L && n_tile == RSlope
                                        || dir == Dir::R && n_tile == LSlope
                                    {
                                        non_wall_tiles += 1;
                                        None
                                    } */

                                    // todo!()
                                })
                                .collect_vec();

                            // dbg!(non_wall_tiles);
                            // dbg!(&open_squares);

                            if non_wall_tiles == 0 || open_squares.is_empty() {
                                break None;
                            } else if non_wall_tiles > 1 {
                                // dbg!(y);
                                // dbg!(x);
                                break Some(Edge {
                                    start: *vertex,
                                    end: Vertex { y, x },
                                    length: num_steps,
                                });
                            }

                            ((y, x), last_dir) = open_squares[0];

                            if y == 0 || y == n_rows - 1 {
                                break Some(Edge {
                                    start: *vertex,
                                    end: Vertex { y, x },
                                    length: num_steps + 1,
                                });
                            }
                            // todo!();
                        }

                        // todo!();
                        // 0
                    })
                    .collect_vec()
                // todo!();
            })
            .collect_vec();

        // dbg!(&edges);

        // let num_vertices = vertices.len();
        // dbg!(num_vertices);
        // let num_edges = edges.len();
        // dbg!(num_edges);

        let graph = DiGraphMap::<Vertex, usize>::from_edges(
            edges.iter().map(|e| (e.start, e.end, e.length)),
        );

        // let g = Dot::with_config(&graph, &[]);
        // dbg!(g);

        let paths: Vec<Vec<_>> =
            all_simple_paths(&graph, starting_vertex, ending_vertex, 0, None).collect_vec();

        let path_lengths = paths
            .iter()
            .map(|path| {
                path.iter()
                    .tuple_windows::<(_, _)>()
                    .map(|(a, b)| graph.edge_weight(*a, *b).unwrap())
                    .sum::<usize>()
            })
            .collect_vec();

        // dbg!(&path_lengths);

        *path_lengths.iter().max().unwrap()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            154
        );
    }
}
