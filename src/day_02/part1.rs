use crate::day_02::count_safe_reports;
use std::cmp::Ordering;

fn is_safe(report: &str, levels: &Vec<i32>) -> bool {
    enum Direction {
        None,
        Increasing,
        Decreasing,
    }

    let mut last = 0;
    let mut direction = Direction::None;

    for &l in levels {
        if last == 0 {
            last = l;
            continue;
        }

        match direction {
            Direction::None => match l.cmp(&last) {
                Ordering::Less => {
                    let diff = last - l;
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                    direction = Direction::Decreasing;
                }
                Ordering::Greater => {
                    let diff = l - last;
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                    direction = Direction::Increasing;
                }
                Ordering::Equal => {
                    return false;
                }
            },
            Direction::Increasing => match l.cmp(&last) {
                Ordering::Less | Ordering::Equal => {
                    return false;
                }
                Ordering::Greater => {
                    let diff = l - last;
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
            },
            Direction::Decreasing => match l.cmp(&last) {
                Ordering::Greater | Ordering::Equal => {
                    return false;
                }
                Ordering::Less => {
                    let diff = last - l;
                    if diff < 1 || diff > 3 {
                        return false;
                    }
                }
            },
        };

        last = l;
    }

    true
}

pub fn part1(input: String) -> usize {
    count_safe_reports(input, is_safe) as usize
}
