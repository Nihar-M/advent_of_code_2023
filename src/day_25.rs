pub const DAY_STR: &str = "day_25";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, newline},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use petgraph::{algo, prelude::*};

    use rand::prelude::*;

    pub fn parse_edges(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
        separated_list1(
            newline,
            separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
        )(input)
    }

    pub fn min_cut_set_size(graph: &UnGraphMap<&str, ()>) -> usize {
        let mut rng = thread_rng();
        let mut residual = graph.clone();

        let (mut start, mut end);

        let nodes = graph.nodes().collect_vec();

        loop {
            // dbg!(attempt);

            (start, end) = nodes
                .choose_multiple(&mut rng, 2)
                .collect_tuple::<(_, _)>()
                .unwrap();

            for _flows in 0..3 {
                let (_len, path) =
                    algo::astar(&residual, start, |e| e == *end, |_| 1, |_| 0).unwrap();
                // dbg!(&path);

                path.into_iter()
                    .tuple_windows::<(_, _)>()
                    .for_each(|(a, b)| {
                        residual.remove_edge(a, b);
                    });
            }

            if algo::astar(&residual, start, |e| e == *end, |_| 1, |_| 0).is_none() {
                // todo!();
                break;
            }

            residual = graph.clone()
        }

        let set_a = nodes
            .iter()
            .copied()
            .filter(|&node| algo::has_path_connecting(&residual, node, start, None))
            .collect_vec();

        // dbg!(&set_a);

        // let edges = graph.edges().collect_vec();

        // {
        //     use petgraph::dot::Dot;
        //     let dot = Dot::with_config(&residual, &[]);
        //     dbg!(dot);
        // }

        // todo!()
        set_a.len()
    }

    pub fn solution(input: String) -> usize {
        let (_input, vertex_map) = parse_edges(&input).unwrap();

        // dbg!(&edges);

        let edges = vertex_map
            .into_iter()
            .flat_map(|(u, vs)| vs.iter().map(|v| (u.clone(), *v)).collect_vec())
            .collect_vec();

        let graph = UnGraphMap::<&str, ()>::from_edges(&edges);

        let set_a_size = min_cut_set_size(&graph);
        // dbg!(set_a_size);

        // {
        //     use petgraph::dot::Dot;
        //     let dot = Dot::with_config(&graph, &[]);
        //     dbg!(dot);
        // }

        // todo!()
        set_a_size * (graph.node_count() - set_a_size)
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            54
        );
    }
}

mod part_2 {

    pub fn solution(_input: String) -> String {
        "Merry Christmas".to_string()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            "Merry Christmas".to_string()
        );
    }
}
