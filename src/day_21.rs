pub const DAY_STR: &str = "day_21";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Tile {
        Wall,
        Odd,
        Even,
        Neither,
    }

    pub fn show_grid(grid: &Vec<Vec<Tile>>) {
        for row in grid {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Wall => '#',
                        Tile::Odd => 'O',
                        Tile::Even => 'E',
                        Tile::Neither => '.',
                    }
                );
            }
            println!();
        }
    }

    pub fn solution(input: String) -> usize {
        let mut fringe = vec![];

        let mut grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, t)| match t {
                        '.' => Tile::Neither,
                        '#' => Tile::Wall,
                        'S' => {
                            fringe.push((y, x));
                            Tile::Even
                        }
                        other => panic!("Unexpected tile {other}"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        // show_grid(&grid);

        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        let max_steps = 65 + 131 + 131;

        for step in 1..=max_steps {
            let mut new_fringe = vec![];

            while let Some((y, x)) = fringe.pop() {
                let new_value = if step % 2 == 0 { Tile::Even } else { Tile::Odd };

                if y > 0 {
                    let new_x = x;
                    let new_y = y - 1;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
                if x > 0 {
                    let new_x = x - 1;
                    let new_y = y;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }

                if y < n_rows - 1 {
                    let new_x = x;
                    let new_y = y + 1;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
                if x < n_cols - 1 {
                    let new_x = x + 1;
                    let new_y = y;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
            }
            fringe = new_fringe;
        }

        // show_grid(&grid);

        grid.iter()
            .map(|row| row.iter().filter(|&tile| tile == &Tile::Even).count())
            .sum()

        // dbg!(&fringe);

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            42
        );
    }

    // requires to change the tile== &Tile::Even to &Tile::Odd

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(
    //         solution(
    //             std::fs::read_to_string(format!("inputs/{}/part_1/part2_1.txt", super::DAY_STR))
    //                 .unwrap()
    //         ),
    //         // 95103
    //         95900
    //     );
    // }
}

mod part_2 {
    use itertools::Itertools;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Tile {
        Wall,
        Both,
        Odd,
        Even,
        Neither,
    }

    pub fn show_grid(grid: &Vec<Vec<Tile>>) {
        for row in grid {
            for tile in row {
                print!(
                    "{}",
                    match tile {
                        Tile::Wall => '#',
                        Tile::Both => 'B',
                        Tile::Odd => 'O',
                        Tile::Even => 'E',
                        Tile::Neither => '.',
                    }
                );
            }
            println!();
        }
    }

    fn do_steps(mut grid: Vec<Vec<Tile>>, start: (usize, usize), max_steps: i32) -> Vec<Vec<Tile>> {
        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        let mut fringe = vec![start];
        for step in 1..=max_steps {
            let mut new_fringe = vec![];

            while let Some((y, x)) = fringe.pop() {
                let new_value = if step % 2 == 0 { Tile::Even } else { Tile::Odd };

                if y > 0 {
                    let new_x = x;
                    let new_y = y - 1;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
                if x > 0 {
                    let new_x = x - 1;
                    let new_y = y;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }

                if y < n_rows - 1 {
                    let new_x = x;
                    let new_y = y + 1;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
                if x < n_cols - 1 {
                    let new_x = x + 1;
                    let new_y = y;
                    let tile = &mut grid[new_y][new_x];
                    if tile == &Tile::Neither {
                        *tile = new_value;
                        new_fringe.push((new_y, new_x));
                    }
                }
            }
            fringe = new_fringe;
        }
        grid
    }

    fn count_tiles(grid: &Vec<Vec<Tile>>, tile: Tile) -> u64 {
        grid.iter()
            .map(|row| row.iter().filter(|&&t| t == tile).count() as u64)
            .sum()
    }

    fn count_odd(grid: &Vec<Vec<Tile>>) -> u64 {
        count_tiles(grid, Tile::Odd)
    }

    fn flip_grid(grid: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
        grid.iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Odd => Tile::Even,
                        Tile::Even => Tile::Odd,
                        other => *other,
                    })
                    .collect_vec()
            })
            .collect_vec()
    }

    fn nth_tri(x: u64) -> u64 {
        x * (x + 1) / 2
    }

    pub fn solution(input: String) -> u64 {
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|t| match t {
                        '.' => Tile::Neither,
                        '#' => Tile::Wall,
                        'S' => Tile::Neither,
                        other => panic!("Unexpected tile {other}"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        // show_grid(&grid);

        // grid is now saturated with every possible step.

        // directly adjacent grids have Evens and Odds flipped
        // diagonally adjacent grids have same Evens and Odds

        // also the map is such that the borders are all empty
        // and that the cardinal directions from the Source
        // to the edges are also all clear

        // the source is also at the center of the grid at (65,65)
        // and the grid is of size (131, 131)

        // so based on this we can find all the repeated "interior"
        // grids which are all completely filled in

        // and we can find out what the 4 diagonal edges look like
        // and finally we can find out what the 4 corners look like

        let max_steps: u64 = 26501365;
        // let max_steps: u64 = 65 + 131 + 131;
        let grid_size: u64 = 131;
        // let st_edge: u64 = 65;
        // let st_corner: u64 = 130;

        let max_repeats = max_steps.div_euclid(grid_size) - 1;
        // let max_repeats = 0;
        // dbg!(max_repeats);
        // let left_over_steps = max_steps % grid_size;
        // dbg!(left_over_steps);

        let all_fully_covered = 4 * nth_tri(max_repeats) + 1;
        // dbg!(all_fully_covered);

        // we assume that max_repeats is even (since it is)

        let num_same = 4 * (2 * nth_tri(max_repeats / 2)) + 1;
        // dbg!(num_same);

        let num_diff = all_fully_covered - num_same;
        // dbg!(num_diff);

        let num_small_edges = max_repeats + 1;
        // dbg!(num_small_edges);
        let num_big_edges = num_small_edges - 1;
        // dbg!(num_big_edges);
        let num_corners = 1;
        // dbg!(num_corners);

        let same_grid = do_steps(grid.clone(), (65, 65), 130);
        // show_grid(&same_grid);

        let diff_grid = flip_grid(same_grid.clone());
        // show_grid(&flipped_grid);

        let u_grid = flip_grid(do_steps(grid.clone(), (130, 65), 65 + 65));
        let d_grid = flip_grid(do_steps(grid.clone(), (0, 65), 65 + 65));
        let l_grid = flip_grid(do_steps(grid.clone(), (65, 130), 65 + 65));
        let r_grid = flip_grid(do_steps(grid.clone(), (65, 0), 65 + 65));

        let ul_grid_small = flip_grid(do_steps(grid.clone(), (130, 130), 65 - 1));
        let ur_grid_small = flip_grid(do_steps(grid.clone(), (130, 0), 65 - 1));
        let dl_grid_small = flip_grid(do_steps(grid.clone(), (0, 130), 65 - 1));
        let dr_grid_small = flip_grid(do_steps(grid.clone(), (0, 0), 65 - 1));

        let ul_grid_big = do_steps(grid.clone(), (130, 130), 130 + 65);
        let ur_grid_big = do_steps(grid.clone(), (130, 0), 130 + 65);
        let dl_grid_big = do_steps(grid.clone(), (0, 130), 130 + 65);
        let dr_grid_big = do_steps(grid.clone(), (0, 0), 130 + 65);

        // show_grid(&ul_grid_small);
        // show_grid(&ul_grid_big);
        // show_grid(&ur_grid_small);
        // show_grid(&r_grid);

        num_same * /* dbg! */(count_odd(&same_grid))
            + num_diff * /* dbg! */(count_odd(&diff_grid))
            + num_corners
                * (/* dbg! */(count_odd(&u_grid))
                    + /* dbg! */(count_odd(&d_grid))
                    + /* dbg! */(count_odd(&l_grid))
                    + /* dbg! */(count_odd(&r_grid)))
            + num_small_edges
                * (/* dbg! */(count_odd(&ul_grid_small))
                    + /* dbg! */(count_odd(&ur_grid_small))
                    + /* dbg! */(count_odd(&dl_grid_small))
                    + /* dbg! */(count_odd(&dr_grid_small)))
            + num_big_edges
                * (/* dbg! */(count_odd(&ul_grid_big))
                    + /* dbg! */(count_odd(&ur_grid_big))
                    + /* dbg! */(count_odd(&dl_grid_big))
                    + /* dbg! */(count_odd(&dr_grid_big)))

        // todo!()

        // ......X......
        // .....X6X.....
        // ....X6X6X.....
        // ...X6X6X6X....
        // ..X6X6X6X6X...
        // .X6X6X6X6X6X..
        // X6X6X6X6X6X6X.
        // .X6X6X6X6X6X..
        // ..X6X6X6X6X...
        // ...X6X6X6X....
        // ....X6X6X.....
        // .....X6X.....
        // ......X......

        // ....X....
        // ...X4X...
        // ..X4X4X..
        // .X4X4X4X.
        // X4X4X4X4X
        // .X4X4X4X.
        // ..X4X4X..
        // ...X4X...
        // ....X....

        // ..X..
        // .X2X.
        // X2X2X
        // .X2X.
        // ..X..
    }

    #[test]
    fn run_on_real_input() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/input.txt", super::DAY_STR))
                    .unwrap()
            ),
            625382480005896
        );
    }
}
