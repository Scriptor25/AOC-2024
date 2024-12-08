use regex::Regex;

pub fn part2(input: String) -> usize {
    let reg =
        Regex::new(r"(mul\((0|([1-9][0-9]*)),(0|([1-9][0-9]*))\))|(do\(\))|(don't\(\))").unwrap();

    let mut sum = 0;
    let mut do_mul = true;

    for cap in reg.captures_iter(input.as_str()) {
        let c = &cap[0];
        if c.starts_with("mul") {
            if do_mul {
                let x_str = &cap[2];
                let y_str = &cap[4];
                let x: i32 = x_str.parse().expect("failed to parse x");
                let y: i32 = y_str.parse().expect("failed to parse y");
                sum += x * y;
            }
        } else if c.starts_with("don't") {
            do_mul = false;
        } else if c.starts_with("do") {
            do_mul = true;
        } else {
            panic!("unhandled match {c}");
        }
    }

    sum as usize
}
