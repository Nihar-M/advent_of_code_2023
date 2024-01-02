pub const DAY_STR: &str = "day_03";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    pub fn solution(input: String) -> u32 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // let connected = Vec::new();

        let row_sz = grid.len();
        // dbg!(row_sz);
        let col_sz = grid[0].len();
        // dbg!(col_sz);

        let mut numbers = vec![];

        let mut j = 0;
        while j < col_sz {
            let mut i = 0;
            while i < row_sz {
                let cell = grid[j][i];

                if cell.is_ascii_digit() {
                    // dbg!(cell);
                    if i == 0 || !grid[j][i - 1].is_ascii_digit() {
                        // is first digit of a number

                        let mut number = 0;

                        let mut connected = false;

                        while i < row_sz {
                            let cell = grid[j][i];
                            // dbg!(number);
                            if cell.is_ascii_digit() {
                                number = number * 10 + cell.to_digit(10).unwrap();

                                let is_connected = |x: usize, y: usize| {
                                    let ch = grid[x][y];
                                    // dbg!(x);
                                    // dbg!(y);
                                    // dbg!(ch);
                                    match ch {
                                        c if c.is_ascii_digit() => false,
                                        '.' => false,
                                        _ => true,
                                    }
                                };

                                // dbg!(i);
                                // dbg!(j);
                                // dbg!(cell);

                                if i > 0 && j > 0 && is_connected(j - 1, i - 1) {
                                    connected = true;
                                }
                                if i > 0 && is_connected(j, i - 1) {
                                    connected = true;
                                }
                                if i > 0 && j < col_sz - 1 && is_connected(j + 1, i - 1) {
                                    connected = true;
                                }
                                if j > 0 && is_connected(j - 1, i) {
                                    connected = true;
                                }
                                if j < col_sz - 1 && is_connected(j + 1, i) {
                                    connected = true;
                                }
                                if i < row_sz - 1 && j > 0 && is_connected(j - 1, i + 1) {
                                    connected = true;
                                }
                                if i < row_sz - 1 && is_connected(j, i + 1) {
                                    connected = true;
                                }
                                if i < row_sz - 1 && j < col_sz - 1 && is_connected(j + 1, i + 1) {
                                    connected = true;
                                }
                            } else {
                                break;
                            }
                            i += 1;
                        }
                        // dbg!(connected);
                        // dbg!(number);

                        if connected {
                            numbers.push(number);
                        }
                    } else {
                        panic!("What went wrong");
                    }
                }
                i += 1;
            }
            j += 1;
        }
        // dbg!(&numbers);

        // dbg!(grid);
        // todo!();
        numbers.iter().sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            4361
        );
    }
}

mod part_2 {
    use std::collections::HashSet;

    pub fn solution(input: String) -> u32 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        // let connected = Vec::new();

        let row_sz = grid.len();
        // dbg!(row_sz);
        let col_sz = grid[0].len();
        // dbg!(col_sz);

        let mut gears = vec![];

        let mut j = 0;
        while j < col_sz {
            let mut i = 0;
            while i < row_sz {
                let cell = grid[j][i];

                let is_number = |y: usize, x: usize| {
                    let ch = grid[y][x];
                    // dbg!(x,y,ch);
                    ch.is_ascii_digit()
                };

                let find_number_range = |j: usize, i: usize| {
                    let mut start = i;
                    let mut end = i;
                    while start > 0 {
                        if is_number(j, start - 1) {
                            start -= 1;
                        } else {
                            break;
                        }
                    }
                    while end < row_sz - 1 {
                        if is_number(j, end + 1) {
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    (j, start..=end)
                };

                // dbg!(i,j,cell);

                if cell == '*' {
                    let mut numbers = HashSet::new();

                    if i > 0 && j > 0 && is_number(j - 1, i - 1) {
                        numbers.insert(find_number_range(j - 1, i - 1));
                    }
                    if i > 0 && is_number(j, i - 1) {
                        numbers.insert(find_number_range(j, i - 1));
                    }
                    if i > 0 && j < col_sz - 1 && is_number(j + 1, i - 1) {
                        numbers.insert(find_number_range(j + 1, i - 1));
                    }
                    if j > 0 && is_number(j - 1, i) {
                        numbers.insert(find_number_range(j - 1, i));
                    }
                    if j < col_sz - 1 && is_number(j + 1, i) {
                        numbers.insert(find_number_range(j + 1, i));
                    }
                    if i < row_sz - 1 && j > 0 && is_number(j - 1, i + 1) {
                        numbers.insert(find_number_range(j - 1, i + 1));
                    }
                    if i < row_sz - 1 && is_number(j, i + 1) {
                        numbers.insert(find_number_range(j, i + 1));
                    }
                    if i < row_sz - 1 && j < col_sz - 1 && is_number(j + 1, i + 1) {
                        numbers.insert(find_number_range(j + 1, i + 1));
                    }
                    // dbg!(i, j, &numbers);

                    if numbers.len() == 2 {
                        let mut numbers_iter = numbers.into_iter();
                        let (f_j, f_range) = numbers_iter.next().unwrap();
                        let (l_j, l_range) = numbers_iter.next().unwrap();
                        let f_num = grid[f_j][f_range]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();
                        let l_num = grid[l_j][l_range]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .unwrap();

                        // dbg!(f_num, l_num);
                        gears.push((f_num, l_num));
                    }
                }

                i += 1;
            }
            j += 1;
        }
        // dbg!(&gears);

        // dbg!(grid);
        gears
            .iter()
            .map(|(f, l)| f * l)
            .inspect(|_x| {
                // dbg!(_x);
            })
            .sum()
        // todo!();
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            467835
        );
    }
}
