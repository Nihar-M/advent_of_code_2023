pub const DAY_STR: &str = "day_02";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete,
        multi::separated_list1,
        sequence::{delimited, preceded, tuple},
        IResult,
    };

    #[derive(Debug)]
    pub struct Round {
        pub red: u32,
        pub green: u32,
        pub blue: u32,
    }

    fn parse_round(input: &str) -> IResult<&str, Round> {
        let mut round = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        // dbg!(input);
        let (input, cubes) = separated_list1(
            tag(","),
            preceded(
                tag(" "),
                tuple((
                    complete::u32,
                    preceded(tag(" "), alt((tag("red"), tag("blue"), tag("green")))),
                )),
            ),
        )(input)?;

        for cube in cubes {
            match cube {
                (x, "red") => round.red += x,
                (x, "green") => round.green += x,
                (x, "blue") => round.blue += x,
                (_, _) => unreachable!(),
            }
        }
        // dbg!(&round);
        Ok((input, round))
    }

    pub fn parse_line(input: &str) -> IResult<&str, (u32, Vec<Round>)> {
        let (input, game_id) = delimited(tag("Game "), complete::u32, tag(":"))(input)?;

        let (input, rounds) = separated_list1(tag(";"), parse_round)(input)?;
        Ok((input, (game_id, rounds)))
    }

    pub fn solution(input: String) -> u32 {
        let contents = Round {
            red: 12,
            green: 13,
            blue: 14,
        };

        let x = input
            .lines()
            .filter_map(|line| {
                let (_, (game_id, rounds)) = parse_line(line).unwrap();

                if rounds.iter().all(|round| {
                    round.red <= contents.red
                        && round.green <= contents.green
                        && round.blue <= contents.blue
                }) {
                    Some(game_id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // dbg!(&x);

        x.iter().sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            8
        );
    }
}

mod part_2 {

    use super::part_1::parse_line;
    use super::part_1::Round;
    use std::cmp::max;

    pub fn solution(input: String) -> u32 {
        let x = input
            .lines()
            .map(|line| {
                let (_, (_, rounds)) = parse_line(line).unwrap();

                let mut min_required = Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                rounds.iter().for_each(|round| {
                    min_required.red = max(min_required.red, round.red);
                    min_required.green = max(min_required.green, round.green);
                    min_required.blue = max(min_required.blue, round.blue);
                });

                min_required.red * min_required.green * min_required.blue
            })
            .collect::<Vec<_>>();

        // dbg!(&x);

        x.iter().sum()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            2286
        );
    }
}
