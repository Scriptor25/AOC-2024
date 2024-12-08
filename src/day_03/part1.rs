use regex::Regex;

pub fn part1(input: String) -> usize {
    let reg = Regex::new(r"mul\((0|([1-9][0-9]*)),(0|([1-9][0-9]*))\)").unwrap();

    let mut sum = 0;

    for cap in reg.captures_iter(input.as_str()) {
        let x_str = &cap[1];
        let y_str = &cap[3];
        let x: i32 = x_str.parse().expect("failed to parse x");
        let y: i32 = y_str.parse().expect("failed to parse y");
        sum += x * y;
    }

    sum as usize
}
