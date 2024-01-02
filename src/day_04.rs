pub const DAY_STR: &str = "day_04";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::{collections::HashSet, usize};

    use nom::{
        self,
        bytes::complete::tag,
        character::complete::{self, multispace1},
        multi::separated_list1,
        IResult,
    };

    fn parse_line_header(input: &str) -> IResult<&str, u32> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, game_id) = complete::u32(input)?;
        let (input, _) = tag(":")(input)?;
        Ok((input, game_id))
    }

    fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
        // can try to replace with nom::fold_many1
        let (input, _) = multispace1(input)?;
        let (input, x) = separated_list1(multispace1, complete::u32)(input)?;
        let x = HashSet::from_iter(x);
        Ok((input, x))
    }

    #[derive(Debug)]
    pub struct Game {
        pub id: u32,
        pub winning: HashSet<u32>,
        pub given: HashSet<u32>,
    }

    pub fn parse_line(input: &str) -> IResult<&str, Game> {
        let (input, game_id) = parse_line_header(input)?;
        let (input, winning_numbers) = parse_numbers(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag("|")(input)?;
        let (input, given_numbers) = parse_numbers(input)?;

        Ok((
            input,
            Game {
                id: game_id,
                winning: winning_numbers,
                given: given_numbers,
            },
        ))
    }

    pub fn solution(input: String) -> usize {
        input
            .lines()
            .map(|line| {
                let (_, game) = parse_line(line).unwrap();
                // dbg!(&game.id);
                let matches = game.winning.intersection(&game.given).count();
                // dbg!(matches);
                match matches {
                    0 => 0,
                    x => 2_usize.pow((x as u32) - 1),
                }
            })
            .sum::<usize>()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            13
        );
    }
}

mod part_2 {

    use super::part_1::parse_line;

    use nom::character::complete::newline;
    use nom::multi::separated_list1;

    pub fn solution(input: String) -> usize {
        let (_input, games) = separated_list1(newline, parse_line)(&input).unwrap();

        let mut games = games.iter().map(|game| (1, game)).collect::<Vec<_>>();
        let mut sum = 0;

        // for (num, game) in games.iter_mut() {
        for i in 0..games.len() {
            // dbg!(num, &game);
            let (num, game) = games[i];
            let matches = game.winning.intersection(&game.given).count();

            let winning_range = (game.id as usize)..(matches + game.id as usize);
            // dbg!(&winning_range);
            for card_id in winning_range {
                // dbg!(games[card_id].1.id);
                games[card_id].0 += num;
            }
            sum += num;
        }

        sum
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            30
        );
    }
}
