use crate::day_07::map_operands;

fn get_bit(i: usize, n: usize) -> bool {
    ((i >> n) & 0b1) != 0
}

pub fn part1(input: String) -> usize {
    let mut total = 0;

    map_operands(input).iter().for_each(|(result, operands)| {
        let len = operands.len();
        for op in 0..2usize.pow(len as u32) {
            let mut k = operands[0];
            for j in 1..len {
                if get_bit(op, j - 1) {
                    k *= operands[j];
                } else {
                    k += operands[j];
                }
            }
            if k == *result {
                total += result;
                println!("{result}");
                break;
            }
        }
    });

    total
}
