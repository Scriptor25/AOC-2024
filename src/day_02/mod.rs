pub mod part1;
pub mod part2;

fn count_safe_reports(input: String, is_safe: fn(&str, &Vec<i32>) -> bool) -> i32 {
    let mut num_safe_reports = 0;

    input.lines().for_each(|report| {
        let levels = report
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if is_safe(report, &levels) {
            num_safe_reports += 1;
        } else {
        }
    });

    num_safe_reports
}
