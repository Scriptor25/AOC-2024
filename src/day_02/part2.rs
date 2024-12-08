use crate::day_02::count_safe_reports;

fn is_safe(report: &str, levels: &Vec<i32>) -> bool {
    let mut last = 0;

    let mut num_inc = 0;
    let mut num_dec = 0;
    let mut num_unsafe = 0;

    for &l in levels {
        if last != 0 {
            let delta = l - last;
            let diff = delta.abs();
            if diff < 1 || diff > 3 {
                num_unsafe += 1;
                continue;
            }
            if delta < 0 {
                num_dec += 1;
            } else {
                num_inc += 1;
            }
        }
        last = l;
    }

    if num_unsafe > 1 {
        return false; // too many unsafe (diff is <1 or >3)
    }

    let min_safe = levels.len() - 2;
    if num_inc > num_dec {
        if num_inc < min_safe {
            return false; // too many not incrementing
        }
    } else if num_dec > num_inc {
        if num_dec < min_safe {
            return false; // too many not decrementing
        }
    }

    println!(
        "{:30} -> inc: {num_inc}, dec: {num_dec}, unsafe: {num_unsafe}",
        report
    );

    true
}

pub fn part2(input: String) -> usize {
    count_safe_reports(input, is_safe) as usize
}
