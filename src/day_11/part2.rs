use std::collections::HashMap;

pub fn part2(input: String) -> usize {
    let mut stones: HashMap<u64, usize> = HashMap::new();

    for stone in input.split_whitespace().map(|s| s.parse::<u64>().unwrap()) {
        *stones.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_stones = HashMap::new();

        for (&stone, &count) in &stones {
            if stone == 0 {
                *new_stones.entry(1).or_insert(0) += count;
            } else if has_even_digits(stone) {
                let (left, right) = split_even_digits(stone);
                *new_stones.entry(left).or_insert(0) += count;
                *new_stones.entry(right).or_insert(0) += count;
            } else {
                *new_stones.entry(stone * 2024).or_insert(0) += count;
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

fn has_even_digits(n: u64) -> bool {
    let digits = n.to_string().len();
    digits % 2 == 0
}

fn split_even_digits(n: u64) -> (u64, u64) {
    let digits = n.to_string();
    let mid = digits.len() / 2;
    let left = digits[..mid].parse::<u64>().unwrap_or(0);
    let right = digits[mid..].parse::<u64>().unwrap_or(0);
    (left, right)
}
