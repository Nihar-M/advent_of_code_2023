#![allow(dead_code, clippy::needless_range_loop, clippy::ptr_arg)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use std::env;

use day_01 as day;

fn main() {
    let day_parts = env::args().collect::<Vec<_>>();

    if day_parts.contains(&"all".to_string()) {
        for day in 1..=25 {
            for part in 1..=2 {
                run_solution(day, part);
            }
        }
    } else {
        for day_part in day_parts {
            if let Some((day, part)) = day_part.split_once('-') {
                run_solution(
                    day.parse().expect("Failed to parse day"),
                    part.parse().expect("Failed to parse part"),
                );
            }
        }
    }
}

fn run_solution(day: u32, part: u32) {
    assert!((1..=25).contains(&day));
    assert!((1..=2).contains(&part));
    let input =
        std::fs::read_to_string(format!("inputs/day_{:02}/part_{}/input.txt", day, part)).unwrap();

    let name = format!("{:02}-{}", day, part);

    match (day, part) {
        (1, 1) => println!("{}: {}", name, day_01::part_1(input)),
        (1, 2) => println!("{}: {}", name, day_01::part_2(input)),
        (2, 1) => println!("{}: {}", name, day_02::part_1(input)),
        (2, 2) => println!("{}: {}", name, day_02::part_2(input)),
        (3, 1) => println!("{}: {}", name, day_03::part_1(input)),
        (3, 2) => println!("{}: {}", name, day_03::part_2(input)),
        (4, 1) => println!("{}: {}", name, day_04::part_1(input)),
        (4, 2) => println!("{}: {}", name, day_04::part_2(input)),
        (5, 1) => println!("{}: {}", name, day_05::part_1(input)),
        (5, 2) => println!("{}: {}", name, day_05::part_2(input)),
        (6, 1) => println!("{}: {}", name, day_06::part_1(input)),
        (6, 2) => println!("{}: {}", name, day_06::part_2(input)),
        (7, 1) => println!("{}: {}", name, day_07::part_1(input)),
        (7, 2) => println!("{}: {}", name, day_07::part_2(input)),
        (8, 1) => println!("{}: {}", name, day_08::part_1(input)),
        (8, 2) => println!("{}: {}", name, day_08::part_2(input)),
        (9, 1) => println!("{}: {}", name, day_09::part_1(input)),
        (9, 2) => println!("{}: {}", name, day_09::part_2(input)),
        (10, 1) => println!("{}: {}", name, day_10::part_1(input)),
        (10, 2) => println!("{}: {}", name, day_10::part_2(input)),
        (11, 1) => println!("{}: {}", name, day_11::part_1(input)),
        (11, 2) => println!("{}: {}", name, day_11::part_2(input)),
        (12, 1) => println!("{}: {}", name, day_12::part_1(input)),
        (12, 2) => println!("{}: {}", name, day_12::part_2(input)),
        (13, 1) => println!("{}: {}", name, day_13::part_1(input)),
        (13, 2) => println!("{}: {}", name, day_13::part_2(input)),
        (14, 1) => println!("{}: {}", name, day_14::part_1(input)),
        (14, 2) => println!("{}: {}", name, day_14::part_2(input)),
        (15, 1) => println!("{}: {}", name, day_15::part_1(input)),
        (15, 2) => println!("{}: {}", name, day_15::part_2(input)),
        (16, 1) => println!("{}: {}", name, day_16::part_1(input)),
        (16, 2) => println!("{}: {}", name, day_16::part_2(input)),
        (17, 1) => println!("{}: {}", name, day_17::part_1(input)),
        (17, 2) => println!("{}: {}", name, day_17::part_2(input)),
        (18, 1) => println!("{}: {}", name, day_18::part_1(input)),
        (18, 2) => println!("{}: {}", name, day_18::part_2(input)),
        (19, 1) => println!("{}: {}", name, day_19::part_1(input)),
        (19, 2) => println!("{}: {}", name, day_19::part_2(input)),
        (20, 1) => println!("{}: {}", name, day_20::part_1(input)),
        (20, 2) => println!("{}: {}", name, day_20::part_2(input)),
        (21, 1) => println!("{}: {}", name, day_21::part_1(input)),
        (21, 2) => println!("{}: {}", name, day_21::part_2(input)),
        (22, 1) => println!("{}: {}", name, day_22::part_1(input)),
        (22, 2) => println!("{}: {}", name, day_22::part_2(input)),
        (23, 1) => println!("{}: {}", name, day_23::part_1(input)),
        (23, 2) => println!("{}: {}", name, day_23::part_2(input)),
        (24, 1) => println!("{}: {}", name, day_24::part_1(input)),
        (24, 2) => println!("{}: {}", name, day_24::part_2(input)),
        (25, 1) => println!("{}: {}", name, day_25::part_1(input)),
        (25, 2) => println!("{}: {}", name, day_25::part_2(input)),

        (other_day, other_part) => panic!("Invalid day: {other_day}, or part: {other_part}"),
    }
}

fn run_part_1() {
    let part_1_input =
        std::fs::read_to_string(format!("inputs/{}/part_1/input.txt", day::DAY_STR)).unwrap();
    let answer = day::part_1(part_1_input);
    println!("Part 1: {}", answer);
}

fn run_part_2() {
    let part_2_input =
        std::fs::read_to_string(format!("inputs/{}/part_2/input.txt", day::DAY_STR)).unwrap();
    let answer = day::part_2(part_2_input);
    println!("Part 2: {}", answer);
}
