use crate::day_07::map_operands;

fn get_digit(mut op: usize, i: usize) -> usize {
    let mut digits = Vec::new();

    while op > 0 {
        digits.push(op % 3);
        op /= 3;
    }

    if i < digits.len() {
        digits[i]
    } else {
        0
    }
}

pub fn part2(input: String) -> usize {
    let mut total = 0;

    map_operands(input).iter().for_each(|(result, operands)| {
        let len = operands.len();
        for op in 0..3usize.pow((len - 1) as u32) {
            let mut k = operands[0];
            for j in 1..len {
                let digit = get_digit(op, j - 1);
                let o = operands[j];
                if digit == 0 {
                    k += o;
                } else if digit == 1 {
                    k *= o;
                } else if digit == 2 {
                    let k_str = k.to_string();
                    let o_str = o.to_string();
                    k = (k_str + &o_str).parse().expect("failed to parse number");
                }
            }
            if k == *result {
                println!("{result}");
                total += result;
                break;
            }
        }
    });

    total
}
