use regex::Regex;

pub fn part2(input: String) -> usize {
    let reg = Regex::new(r"\d+").expect("failed to compile regex");

    let mut stones = reg
        .captures_iter(input.as_str())
        .map(|cap| {
            cap.iter()
                .map(|d| {
                    d.unwrap()
                        .as_str()
                        .parse::<usize>()
                        .expect("failed to parse number")
                })
                .last()
                .unwrap()
        })
        .collect::<Vec<usize>>();

    let mut step = || {
        let mut i = 0;
        while i < stones.len() {
            let stone = stones[i];
            if stone == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let (l_str, r_str) = stone_str.split_at(stone_str.len() / 2);
                let l = l_str.parse::<usize>().expect("failed to parse number");
                let r = r_str.parse::<usize>().expect("failed to parse number");
                stones[i] = l;
                stones.insert(i + 1, r);
                i += 2;
                continue;
            }
            stones[i] = stone * 2024;
            i += 1;
        }
    };

    for _ in 0..75 {
        step();
    }

    stones.len()
}
