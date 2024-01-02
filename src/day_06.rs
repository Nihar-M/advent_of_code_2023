pub const DAY_STR: &str = "day_06";

pub use part_1::solution as part_1;
pub use part_2::solution as part_2;

mod part_1 {
    use std::iter::zip;

    use nom::{
        bytes::complete::{tag, take_until},
        character::complete::{self, multispace1},
        multi::many1,
        sequence::preceded,
        IResult,
    };

    fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
        // dbg!(input);
        let (input, _) = take_until(":")(input)?;
        // dbg!(input);
        let (input, _) = tag(":")(input)?;
        // dbg!(input);
        let (input, x) = many1(preceded(multispace1, complete::u64))(input)?;
        // dbg!(input);
        // todo!()
        Ok((input, x))
    }

    fn distance_traveled(charge_time: u64, total_time: u64) -> u64 {
        assert!(charge_time <= total_time);

        let speed = charge_time;
        let move_time = total_time - charge_time;

        move_time * speed
    }

    pub fn solution(input: String) -> u64 {
        // dbg!(&input);
        let (input, times) = parse_line(&input).unwrap();
        let (_input, records) = parse_line(input).unwrap();

        // dbg!(&times);
        // dbg!(&records);

        let x = zip(times, records)
            .map(|(max_time, record)| {
                let mut ways_beat = 0;
                for t in 0..max_time {
                    let dist = distance_traveled(t, max_time);
                    if dist > record {
                        ways_beat += 1;
                    }
                }
                ways_beat
            })
            .collect::<Vec<_>>();

        // dbg!(&x);

        // todo!()
        x.iter().product()
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_1/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            288
        );
    }

    #[test]
    fn test_dist() {
        assert_eq!(distance_traveled(10, 10), 0);
        assert_eq!(distance_traveled(0, 10), 0);
        assert_eq!(distance_traveled(5, 10), 25);
        assert_eq!(distance_traveled(4, 10), 24);
        assert_eq!(distance_traveled(8, 10), 16);
    }
}

mod part_2 {

    pub fn solution(input: String) -> u64 {
        super::part_1::solution(input)
    }

    #[test]
    fn sample() {
        assert_eq!(
            solution(
                std::fs::read_to_string(format!("inputs/{}/part_2/sample.txt", super::DAY_STR))
                    .unwrap()
            ),
            71503
        );
    }
}
