pub const DAY_STR: &str = "day_08";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{collections::HashMap, fmt::Debug};

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Node(pub [char; 3]);

    impl From<&str> for Node {
        fn from(value: &str) -> Self {
            Node([
                value.chars().nth(0).unwrap(),
                value.chars().nth(1).unwrap(),
                value.chars().nth(2).unwrap(),
            ])
        }
    }

    impl Debug for Node {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{}{}{}", &self.0[0], &self.0[1], &self.0[2]))
        }
    }

    pub fn solution(input: String) -> usize {
        // dbg!(&input);
        let (dirs, input) = &input.split_once("\n\n").unwrap();
        // dbg!(&dirs);
        let (l_map, r_map) = input
            .split_terminator('\n')
            .map(|line| {
                (
                    Node::from(&line[0..3]),
                    Node::from(&line[7..10]),
                    Node::from(&line[12..15]),
                )
            })
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut l_map, mut r_map), edge| {
                    l_map.insert(edge.0, edge.1);
                    r_map.insert(edge.0, edge.2);
                    (l_map, r_map)
                },
            );

        // dbg!("{:?}", &l_map);
        // dbg!("{:?}", &r_map);

        let x = dirs.chars().cycle().enumerate();

        let mut current_node = Node::from("AAA");

        for (iter, dir) in x {
            // dbg!(iter);
            current_node = match dir {
                'L' => *l_map.get(&current_node).unwrap(),
                'R' => *r_map.get(&current_node).unwrap(),
                x => panic!("Unexpected direction {x}"),
            };
            // dbg!(current_node);

            if current_node == Node::from("ZZZ") {
                // dbg!(iter + 1);
                return iter + 1;
            }

            if iter > 100000 {
                break;
            }
        }

        panic!("Failed to find solution within max iterations");
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            2
        );
    }

    #[test]
    fn sample_2() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample2.txt", super::DAY_STR))
                    .unwrap()
            ),
            6
        );
    }
}

mod part_2 {

    use std::collections::HashMap;

    use super::part_1::Node;

    #[derive(Debug, Clone, Copy)]
    struct NodeStatus {
        node: Node,
        last_reached_z: Option<usize>,
        cycle_length: Option<usize>,
    }

    pub fn lcm(a: usize, b: usize) -> usize {
        let mut x;
        let mut y;

        if a > b {
            x = a;
            y = b;
        } else {
            x = b;
            y = a;
        }

        let mut rem = x % y;

        while rem != 0 {
            x = y;
            y = rem;
            rem = x % y;
        }

        a * b / y
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 5), 20);
        assert_eq!(lcm(12, 16), 48);
        assert_eq!(lcm(10, 9), 90);
    }

    pub fn solution(input: String) -> usize {
        // dbg!(&input);
        let (dirs, input) = &input.split_once("\n\n").unwrap();
        // dbg!(&dirs);
        let (l_map, r_map) = input
            .split_terminator('\n')
            .map(|line| {
                (
                    Node::from(&line[0..3]),
                    Node::from(&line[7..10]),
                    Node::from(&line[12..15]),
                )
            })
            .fold(
                (HashMap::new(), HashMap::new()),
                |(mut l_map, mut r_map), edge| {
                    l_map.insert(edge.0, edge.1);
                    r_map.insert(edge.0, edge.2);
                    (l_map, r_map)
                },
            );

        // dbg!("{:?}", &l_map);
        // dbg!("{:?}", &r_map);

        let x = dirs
            .chars()
            .cycle()
            .enumerate()
            .map(|(idx, x)| (idx + 1, x));

        // let mut current_node = Node::from("AAA");

        let mut current_nodes = l_map
            .keys()
            .filter(|node| node.0[2] == 'A')
            .map(|node| NodeStatus {
                node: *node,
                last_reached_z: None,
                cycle_length: None,
            })
            .collect::<Vec<_>>();

        // dbg!(&current_nodes);

        for (iter, dir) in x {
            // dbg!(iter);

            current_nodes.iter_mut().for_each(|ns| {
                if ns.cycle_length.is_none() {
                    ns.node = match dir {
                        'L' => *l_map.get(&ns.node).unwrap(),
                        'R' => *r_map.get(&ns.node).unwrap(),
                        x => panic!("Unexpected direction {x}"),
                    };

                    if ns.node.0[2] == 'Z' {
                        match ns.last_reached_z {
                            Some(x) => ns.cycle_length = Some(iter - x),
                            None => ns.last_reached_z = Some(iter),
                        }
                    }
                }
            });

            // dbg!(&current_nodes);

            if current_nodes.iter().all(|ns| ns.cycle_length.is_some()) {
                // dbg!("Broke loop at", iter);
                break;
            }

            if iter > 100000 {
                break;
            }
        }

        // dbg!(&current_nodes);

        // the last_reached_z is equal to the cycle length for all nodes
        current_nodes.iter().for_each(|ns| {
            assert_eq!(ns.last_reached_z, ns.cycle_length);
        });

        let cycle_lengths = current_nodes
            .iter()
            .map(|ns| ns.cycle_length.unwrap())
            .collect::<Vec<_>>();

        // cycle_lengths

        // dbg!(x);

        cycle_lengths.into_iter().reduce(lcm).unwrap()

        // todo!();
        // panic!("Failed to find solution within max iterations");
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            6
        );
    }
}
