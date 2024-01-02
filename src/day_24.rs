pub const DAY_STR: &str = "day_24";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::fmt::Debug;

    use itertools::Itertools;
    use nom::{
        bytes::complete::tag,
        character::complete::{self, newline},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct I64Vec3 {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    impl Debug for I64Vec3 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "Vec3U64 ( x : {}, y : {}, z : {} )",
                self.x, self.y, self.z
            ))
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Ball {
        pub pos: I64Vec3,
        pub vel: I64Vec3,
    }

    pub fn parse_balls(input: &str) -> IResult<&str, Vec<Ball>> {
        let (input, separated) = separated_list1(
            newline,
            tuple((
                complete::i64,
                tag(", "),
                complete::i64,
                tag(", "),
                complete::i64,
                tag(" @ "),
                complete::i64,
                tag(", "),
                complete::i64,
                tag(", "),
                complete::i64,
            )),
        )(input)?;

        Ok((
            input,
            separated
                .into_iter()
                .map(|(px, _, py, _, pz, _, vx, _, vy, _, vz)| Ball {
                    pos: I64Vec3 {
                        x: px,
                        y: py,
                        z: pz,
                    },
                    vel: I64Vec3 {
                        x: vx,
                        y: vy,
                        z: vz,
                    },
                })
                .collect_vec(),
        ))
    }

    pub fn solution(input: String) -> usize {
        let (_input, balls) = parse_balls(&input).unwrap();

        // dbg!(_input);

        // dbg!(&balls);
        // dbg!(balls.len());

        let z = balls
            .iter()
            .combinations(2)
            .map(|pair| {
                let one = pair[0];
                let two = pair[1];

                // dbg!(one);
                // dbg!(two);

                // [ v_x2, - v_x1 ] [t_2]  =  [p_x1 - p_x2]
                // [ v_y2, - v_y1 ] [t_1]     [p_y1 - p_y2]

                // if a solution exists then this
                // matrix equation will have a solution
                // for some t_2, t_1 (positive)
                // then we can use them to check if
                // the intersection lies in the range

                let v_x1 = one.vel.x as f64;
                let v_y1 = one.vel.y as f64;
                let p_x1 = one.pos.x as f64;
                let p_y1 = one.pos.y as f64;

                let v_x2 = two.vel.x as f64;
                let v_y2 = two.vel.y as f64;
                let p_x2 = two.pos.x as f64;
                let p_y2 = two.pos.y as f64;

                // first find the determinant of the matrix
                // det [a b]   = ad - bc
                //     [c d]
                let det = v_x2 * (-v_y1) - (-v_x1) * v_y2;

                // dbg!(det);

                if det == 0.0 {
                    return None;
                }

                // now we can invert the matrix to get
                //[t_2]  = (1 /   ) [ -v_y1, v_x1 ]  [p_x1 - p_x2]
                //[t_1]    ( / det) [ -v_y2, v_x2 ]  [p_y1 - p_y2]

                let d_px = p_x1 - p_x2;
                let d_py = p_y1 - p_y2;

                let t_2 = (1.0 / det) * ((-v_y1) * d_px + (v_x1) * d_py);
                let t_1 = (1.0 / det) * ((-v_y2) * d_px + (v_x2) * d_py);

                // dbg!(t_2);
                // dbg!(t_1);

                if t_1 < 0.0 || t_2 < 0.0 {
                    return None;
                }

                let intersect_x = p_x1 + v_x1 * t_1;
                let intersect_y = p_y1 + v_y1 * t_1;

                // dbg!(intersect_x);
                // dbg!(intersect_y);

                if !(200000000000000.0..=400000000000000.0).contains(&intersect_x)
                    || !(200000000000000.0..=400000000000000.0).contains(&intersect_y)
                {
                    return None;
                }

                Some((intersect_x, intersect_y))
            })
            .collect_vec();
        // dbg!(&z);

        z.iter().filter(|x| x.is_some()).count()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            3
        );
    }
}

mod part_2 {

    use itertools::Itertools;

    use ndarray::prelude::*;
    use ndarray_linalg::Solve;

    use super::part_1::*;

    pub fn solution(input: String) -> usize {
        let (_input, balls) = parse_balls(&input).unwrap();

        // dbg!(_input);

        // dbg!(&balls);
        // dbg!(balls.len());

        // Method based on: https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/ketigrg/

        // order of coefficients
        // Voy: Pbx - Pax
        // Pox: Vby - Vay
        // Vox:

        let ball_offset_x = balls.iter().take(5).map(|ball| ball.pos.x).min().unwrap();
        let ball_offset_y = balls.iter().take(5).map(|ball| ball.pos.y).min().unwrap();
        let ball_offset_z = balls.iter().take(5).map(|ball| ball.pos.z).min().unwrap();

        // dbg!(ball_offset_x);
        // dbg!(ball_offset_y);
        // dbg!(ball_offset_z);

        let coefficients = balls
            .iter()
            .copied()
            .take(5)
            .map(|ball| Ball {
                pos: I64Vec3 {
                    x: ball.pos.x - ball_offset_x,
                    y: ball.pos.y - ball_offset_y,
                    z: ball.pos.z - ball_offset_z,
                },
                vel: ball.vel,
            })
            .tuple_windows::<(_, _)>()
            .map(|(one, two)| {
                // dbg!(one);
                // dbg!(two);

                let px_1 = one.pos.x as f64;
                let py_1 = one.pos.y as f64;
                let vx_1 = one.vel.x as f64;
                let vy_1 = one.vel.y as f64;

                let px_2 = two.pos.x as f64;
                let py_2 = two.pos.y as f64;
                let vx_2 = two.vel.x as f64;
                let vy_2 = two.vel.y as f64;

                // (( vy_2 - vy_1), (vx_1 - vx_2), (py_2 - py_1), (px_2 - px_1) ) * (a) = (py_1 * vx_1 - py_2 * vx_2 + px_2 * vy_2 - px_1 * vy_1)
                (
                    ((vy_2 - vy_1), (vx_1 - vx_2), (py_2 - py_1), (px_2 - px_1)),
                    (py_1 * vx_1 - py_2 * vx_2 + px_2 * vy_2 - px_1 * vy_1),
                )
            })
            .collect_vec();

        // dbg!(&coefficients);

        // println!("XY Coefficients");

        // for co_eff in &coefficients {
        //     println!(
        //         "{},{},{},{}",
        //         co_eff.0 .0, co_eff.0 .1, co_eff.0 .2, co_eff.0 .3
        //     );
        // }

        // for co_eff in &coefficients {
        //     println!("{}", co_eff.1);
        // }

        let c = coefficients;

        let coefficients_a: Array2<f64> = array![
            [c[0].0 .0, c[0].0 .1, c[0].0 .2, c[0].0 .3],
            [c[1].0 .0, c[1].0 .1, c[1].0 .2, c[1].0 .3],
            [c[2].0 .0, c[2].0 .1, c[2].0 .2, c[2].0 .3],
            [c[3].0 .0, c[3].0 .1, c[3].0 .2, c[3].0 .3]
        ];

        let coefficients_b: Array1<f64> = array![c[0].1, c[1].1, c[2].1, c[3].1];

        let sol_xy = coefficients_a.solve_into(coefficients_b).unwrap();
        // dbg!(&sol_xy);

        // Do the same for XZ components

        let coefficients = balls
            .iter()
            .take(5)
            .copied()
            .map(|ball| Ball {
                pos: I64Vec3 {
                    x: ball.pos.x - ball_offset_x,
                    y: ball.pos.y - ball_offset_y,
                    z: ball.pos.z - ball_offset_z,
                },
                vel: ball.vel,
            })
            .tuple_windows::<(_, _)>()
            .map(|(one, two)| {
                // dbg!(one);
                // dbg!(two);

                let px_1 = one.pos.x as f64;
                let pz_1 = one.pos.z as f64;
                let vx_1 = one.vel.x as f64;
                let vz_1 = one.vel.z as f64;

                let px_2 = two.pos.x as f64;
                let pz_2 = two.pos.z as f64;
                let vx_2 = two.vel.x as f64;
                let vz_2 = two.vel.z as f64;

                // (( vz_2 - vz_1), (vx_1 - vx_2), (pz_2 - pz_1), (px_2 - px_1) ) * (a) = (pz_1 * vx_1 - pz_2 * vx_2 + px_2 * vz_2 - px_1 * vz_1)
                (
                    ((vz_2 - vz_1), (vx_1 - vx_2), (pz_2 - pz_1), (px_2 - px_1)),
                    (pz_1 * vx_1 - pz_2 * vx_2 + px_2 * vz_2 - px_1 * vz_1),
                )
            })
            .collect_vec();

        // dbg!(&coefficients);

        // println!("XZ Coefficients");

        // for co_eff in &coefficients {
        //     println!(
        //         "{},{},{},{}",
        //         co_eff.0 .0, co_eff.0 .1, co_eff.0 .2, co_eff.0 .3
        //     );
        // }

        // for co_eff in &coefficients {
        //     println!("{}", co_eff.1);
        // }

        let c = coefficients;

        let coefficients_a: Array2<f64> = array![
            [c[0].0 .0, c[0].0 .1, c[0].0 .2, c[0].0 .3],
            [c[1].0 .0, c[1].0 .1, c[1].0 .2, c[1].0 .3],
            [c[2].0 .0, c[2].0 .1, c[2].0 .2, c[2].0 .3],
            [c[3].0 .0, c[3].0 .1, c[3].0 .2, c[3].0 .3]
        ];

        let coefficients_b: Array1<f64> = array![c[0].1, c[1].1, c[2].1, c[3].1];

        let sol_xz = coefficients_a.solve_into(coefficients_b).unwrap();
        // dbg!(&sol_xz);

        let start_x = sol_xy[0] + ball_offset_x as f64;
        let start_y = sol_xy[1] + ball_offset_y as f64;
        let start_z = sol_xz[1] + ball_offset_z as f64;

        // dbg!(start_x);
        // dbg!(start_y);
        // dbg!(start_z);

        let total = start_x + start_y + start_z;
        // dbg!(total);

        total.round() as usize
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            47
        );
    }
}
