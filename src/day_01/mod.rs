pub mod part1;
pub mod part2;

fn split_columns(input: String, a: &mut Vec<i32>, b: &mut Vec<i32>) {
    input.lines().for_each(|line| {
        let numbers = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        a.push(numbers[0]);
        b.push(numbers[1]);
    });
}
