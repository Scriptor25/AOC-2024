use std::fs;
use std::io::{stdin, stdout, Write};

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

fn day_fn(day: i32, part: i32) -> fn(String) -> i32 {
    match day {
        1 => match part {
            1 => day_01::part1::part1,
            2 => day_01::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        2 => match part {
            1 => day_02::part1::part1,
            2 => day_02::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        3 => match part {
            1 => day_03::part1::part1,
            2 => day_03::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        4 => match part {
            1 => day_04::part1::part1,
            2 => day_04::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        5 => match part {
            1 => day_05::part1::part1,
            2 => day_05::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        6 => match part {
            1 => day_06::part1::part1,
            2 => day_06::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        7 => match part {
            1 => day_07::part1::part1,
            2 => day_07::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        8 => match part {
            1 => day_08::part1::part1,
            2 => day_08::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        9 => match part {
            1 => day_09::part1::part1,
            2 => day_09::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        10 => match part {
            1 => day_10::part1::part1,
            2 => day_10::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        11 => match part {
            1 => day_11::part1::part1,
            2 => day_11::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        12 => match part {
            1 => day_12::part1::part1,
            2 => day_12::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        13 => match part {
            1 => day_13::part1::part1,
            2 => day_13::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        14 => match part {
            1 => day_14::part1::part1,
            2 => day_14::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        15 => match part {
            1 => day_15::part1::part1,
            2 => day_15::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        16 => match part {
            1 => day_16::part1::part1,
            2 => day_16::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        17 => match part {
            1 => day_17::part1::part1,
            2 => day_17::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        18 => match part {
            1 => day_18::part1::part1,
            2 => day_18::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        19 => match part {
            1 => day_19::part1::part1,
            2 => day_19::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        20 => match part {
            1 => day_20::part1::part1,
            2 => day_20::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        21 => match part {
            1 => day_21::part1::part1,
            2 => day_21::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        22 => match part {
            1 => day_22::part1::part1,
            2 => day_22::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        23 => match part {
            1 => day_23::part1::part1,
            2 => day_23::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        24 => match part {
            1 => day_24::part1::part1,
            2 => day_24::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        25 => match part {
            1 => day_25::part1::part1,
            2 => day_25::part2::part2,
            _ => panic!("invalid part {part} for day {day}"),
        },
        _ => panic!("invalid day {day}"),
    }
}

fn main() {
    let mut day = String::new();
    let mut part = String::new();

    print!("day: (1..25) ");
    stdout().flush().expect("stdout flush failed");
    stdin().read_line(&mut day).expect("failed to read day");
    {
        let len = day.trim_end_matches(&['\r', '\n'][..]).len();
        day.truncate(len);
    }
    let day_num = day.parse::<i32>().expect("failed to parse day");

    print!("part: (1..2) ");
    stdout().flush().expect("stdout flush failed");
    stdin().read_line(&mut part).expect("failed to read part");
    {
        let len = part.trim_end_matches(&['\r', '\n'][..]).len();
        part.truncate(len);
    }
    let part_num = part.parse::<i32>().expect("failed to parse part");

    let path = format!("input/{:02}/{:02}/input.txt", day_num, part_num);
    let input = fs::read_to_string(path)
        .expect(&format!("input missing for day {day_num} part {part_num}"));

    let result = day_fn(day_num, part_num)(input);
    println!("result: {result}");
}
