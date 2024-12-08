pub mod part1;
pub mod part2;

fn map_operands(input: String) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let operands = line
                .split_whitespace()
                .map(|word| word.replace(":", ""))
                .map(|word| {
                    word.parse::<usize>()
                        .expect(format!("failed to parse operand '{word}'").as_str())
                })
                .collect::<Vec<usize>>();
            (operands[0], operands.split_at(1).1.to_vec())
        })
        .collect()
}
