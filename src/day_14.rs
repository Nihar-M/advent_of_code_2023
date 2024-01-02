pub const DAY_STR: &str = "day_14";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use std::fmt::Debug;

    use itertools::Itertools;

    pub struct Grid(pub Vec<Vec<char>>);
    impl Debug for Grid {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("Grid(")?;
            for row in &self.0 {
                f.write_str("\n\t")?;
                f.write_str(&row.iter().join(""))?;
            }
            f.write_str("\n)")
        }
    }

    pub fn tilt_north(grid: &mut Vec<Vec<char>>) {
        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        for _ in 0..n_rows {
            for j in 0..n_rows - 1 {
                for i in 0..n_cols {
                    if grid[j + 1][i] == 'O' && grid[j][i] == '.' {
                        grid[j][i] = 'O';
                        grid[j + 1][i] = '.';
                    }
                }
            }
        }
    }

    pub fn calculate_north_load(grid: &Vec<Vec<char>>) -> usize {
        let n_rows = grid.len();

        let z = grid
            .iter()
            .enumerate()
            .map(|(idx, row)| row.iter().filter(|x| **x == 'O').count() * (n_rows - idx))
            .collect_vec();
        // dbg!(&z);
        z.iter().sum()
    }

    pub fn solution(input: String) -> usize {
        let mut grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        // dbg!(Grid(grid.clone()));

        tilt_north(&mut grid);
        // dbg!(Grid(grid.clone()));

        calculate_north_load(&grid)
        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            136
        );
    }
}

mod part_2 {

    use itertools::Itertools;

    use super::part_1::calculate_north_load;
    use super::part_1::tilt_north;

    fn tilt_south(grid: &mut Vec<Vec<char>>) {
        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        for _ in 0..n_rows {
            for j in 1..n_rows {
                for i in 0..n_cols {
                    if grid[j - 1][i] == 'O' && grid[j][i] == '.' {
                        grid[j][i] = 'O';
                        grid[j - 1][i] = '.';
                    }
                }
            }
        }
    }

    fn tilt_east(grid: &mut Vec<Vec<char>>) {
        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        for _ in 0..n_cols {
            for j in 1..n_cols {
                for i in 0..n_rows {
                    if grid[i][j - 1] == 'O' && grid[i][j] == '.' {
                        grid[i][j] = 'O';
                        grid[i][j - 1] = '.';
                    }
                }
            }
        }
    }

    fn tilt_west(grid: &mut Vec<Vec<char>>) {
        let n_rows = grid.len();
        let n_cols = grid.first().unwrap().len();

        for _ in 0..n_rows {
            for j in 0..n_rows - 1 {
                for i in 0..n_cols {
                    if grid[i][j + 1] == 'O' && grid[i][j] == '.' {
                        grid[i][j] = 'O';
                        grid[i][j + 1] = '.';
                    }
                }
            }
        }
    }

    fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut g = grid.clone();
        tilt_north(&mut g);
        tilt_west(&mut g);
        tilt_south(&mut g);
        tilt_east(&mut g);
        g
    }

    pub fn solution(input: String) -> usize {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        // dbg!(Grid(grid.clone()));

        let num_runs = 1000000000;

        let mut old_versions = vec![grid];

        loop {
            let last = old_versions.last().unwrap().clone();
            let current = cycle(last);

            if old_versions.contains(&current) {
                // dbg!(&old_versions);
                old_versions.push(current);
                break;
            } else {
                old_versions.push(current);
            }
            // dbg!(&old_versions.len());
        }

        // old_versions
        //     .iter()
        //     .inspect(|x| {
        //         dbg!(Grid(x.to_vec()));
        //     })
        //     .count();
        // dbg!(old_versions.len());

        let last = old_versions.last().unwrap().clone();

        let dupes_indices = old_versions
            .iter()
            .enumerate()
            .filter_map(|(idx, g)| if g == &last { Some(idx) } else { None })
            .collect_vec();

        // dbg!(&dupes_indices);
        assert!(dupes_indices.len() == 2);

        let cycle_start = dupes_indices[0];
        let cycle_end = dupes_indices[1];
        // dbg!(cycle_start);
        // dbg!(cycle_end);

        // let cycle_len = cycle_end - cycle_start;
        // dbg!(cycle_len);

        // let remaining_runs = num_runs - cycle_end;
        // dbg!(remaining_runs);

        // let skipped_runs = remaining_runs.div_euclid(cycle_len);
        // dbg!(skipped_runs);

        // let current_runs = cycle_end + skipped_runs * cycle_len;
        // dbg!(current_runs);

        // let leftover_runs = num_runs - current_runs;
        // dbg!(leftover_runs);

        // let final_state_idx = cycle_start + leftover_runs;
        // dbg!(final_state_idx);

        let final_state_idx_alt = cycle_start + (num_runs - cycle_end) % (cycle_end - cycle_start);
        // dbg!(final_state_idx_alt);

        let final_state = &old_versions[final_state_idx_alt];

        calculate_north_load(final_state)

        // dbg!(Grid(grid.clone()));

        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            64
        );
    }
}
