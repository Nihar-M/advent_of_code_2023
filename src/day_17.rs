pub const DAY_STR: &str = "day_17";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{
        cmp::{min, Reverse},
        collections::{BinaryHeap, HashMap},
    };

    use itertools::Itertools;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Moves {
        UpDown,
        LeftRight,
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub struct Cell {
        pub y_i: usize,
        pub x_i: usize,
        pub next_move: Moves,
    }

    pub fn solution(input: String) -> u32 {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();

        // dbg!(&grid);
        // todo!();

        let mut visited: HashMap<Cell, u32> = HashMap::new();

        let mut fringe: BinaryHeap<Reverse<(u32, Cell)>> = BinaryHeap::new();
        // let mut fringe: BTreeMap<u32, Cell> = BTreeMap::new();

        fringe.push(Reverse((
            0,
            Cell {
                y_i: 0,
                x_i: 0,
                next_move: Moves::LeftRight,
            },
        )));

        fringe.push(Reverse((
            0,
            Cell {
                y_i: 0,
                x_i: 0,
                next_move: Moves::UpDown,
            },
        )));

        while !fringe.is_empty() {
            let Reverse((cost, state)) = fringe.pop().unwrap();

            visited.insert(state, cost);

            // dbg!(cost);
            // dbg!(fringe.len());
            // dbg!(state);

            match state.next_move {
                Moves::UpDown => {
                    let new_next_move = Moves::LeftRight;

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.y_i >= jump {
                            let new_yi = state.y_i - jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.y_i < grid.len() - jump {
                            let new_yi = state.y_i + jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }
                }
                Moves::LeftRight => {
                    let new_next_move = Moves::UpDown;

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.x_i >= jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i - jump;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.x_i < grid.len() - jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i + jump;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }
                }
            }
            // todo!();

            // if visited.len() > 10 {
            //     break;
            // }
        }

        // dbg!(&fringe);
        // dbg!(&visited);

        let ending_lr = visited.get(&Cell {
            y_i: grid.len() - 1,
            x_i: grid.len() - 1,
            next_move: Moves::LeftRight,
        });
        // dbg!(ending_lr);

        let ending_ud = visited.get(&Cell {
            y_i: grid.len() - 1,
            x_i: grid.len() - 1,
            next_move: Moves::UpDown,
        });
        // dbg!(ending_ud);

        let ending = min(ending_lr, ending_ud);
        // dbg!(ending);

        *ending.unwrap()
        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            102
        );
    }
}

mod part_2 {

    use std::{
        cmp::{min, Reverse},
        collections::{BinaryHeap, HashMap},
    };

    use itertools::Itertools;

    use super::part_1::*;

    pub fn solution(input: String) -> u32 {
        let grid = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec();

        // dbg!(&grid);
        // todo!();

        let mut visited: HashMap<Cell, u32> = HashMap::new();

        let mut fringe: BinaryHeap<Reverse<(u32, Cell)>> = BinaryHeap::new();
        // let mut fringe: BTreeMap<u32, Cell> = BTreeMap::new();

        fringe.push(Reverse((
            0,
            Cell {
                y_i: 0,
                x_i: 0,
                next_move: Moves::LeftRight,
            },
        )));

        fringe.push(Reverse((
            0,
            Cell {
                y_i: 0,
                x_i: 0,
                next_move: Moves::UpDown,
            },
        )));

        while !fringe.is_empty() {
            let Reverse((cost, state)) = fringe.pop().unwrap();

            visited.insert(state, cost);

            // dbg!(cost);
            // dbg!(fringe.len());
            // dbg!(state);

            match state.next_move {
                Moves::UpDown => {
                    let new_next_move = Moves::LeftRight;

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.y_i >= jump {
                            let new_yi = state.y_i - jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];
                        }
                    }
                    for jump in 4..=10 {
                        if state.y_i >= jump {
                            let new_yi = state.y_i - jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.y_i < grid.len() - jump {
                            let new_yi = state.y_i + jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];
                        }
                    }
                    for jump in 4..=10 {
                        if state.y_i < grid.len() - jump {
                            let new_yi = state.y_i + jump;
                            let new_xi = state.x_i;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }
                }
                Moves::LeftRight => {
                    let new_next_move = Moves::UpDown;

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.x_i >= jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i - jump;
                            new_cell_cost += grid[new_yi][new_xi];
                        }
                    }
                    for jump in 4..=10 {
                        if state.x_i >= jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i - jump;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }

                    let mut new_cell_cost = cost;
                    for jump in 1..=3 {
                        if state.x_i < grid.len() - jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i + jump;
                            new_cell_cost += grid[new_yi][new_xi];
                        }
                    }
                    for jump in 4..=10 {
                        if state.x_i < grid.len() - jump {
                            let new_yi = state.y_i;
                            let new_xi = state.x_i + jump;
                            new_cell_cost += grid[new_yi][new_xi];

                            let new_cell = Cell {
                                y_i: new_yi,
                                x_i: new_xi,
                                next_move: new_next_move,
                            };
                            // dbg!(new_cell);
                            // dbg!(new_cell_cost);

                            if let Some(old_cost) = visited.get(&new_cell) {
                                assert!(*old_cost <= new_cell_cost);
                            } else {
                                let mut found = false;
                                fringe = fringe
                                    .into_iter()
                                    .map(|Reverse((x_cost, x_cell))| {
                                        if x_cell == new_cell {
                                            found = true;
                                            Reverse((min(x_cost, new_cell_cost), x_cell))
                                        } else {
                                            Reverse((x_cost, x_cell))
                                        }
                                    })
                                    .collect();
                                if !found {
                                    fringe.push(Reverse((new_cell_cost, new_cell)));
                                }
                            }
                        }
                    }
                }
            }
            // todo!();

            // if visited.len() > 10 {
            //     break;
            // }
        }

        // dbg!(&fringe);
        // dbg!(&visited);

        let ending_lr = visited.get(&Cell {
            y_i: grid.len() - 1,
            x_i: grid.len() - 1,
            next_move: Moves::LeftRight,
        });
        // dbg!(ending_lr);

        let ending_ud = visited.get(&Cell {
            y_i: grid.len() - 1,
            x_i: grid.len() - 1,
            next_move: Moves::UpDown,
        });
        // dbg!(ending_ud);

        let ending = min(ending_lr, ending_ud);
        // dbg!(ending);

        *ending.unwrap()
        // todo!()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            94
        );
    }
}
