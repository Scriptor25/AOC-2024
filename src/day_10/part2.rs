use std::collections::VecDeque;

pub fn part2(input: String) -> usize {
    let topographic_map = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 0x30).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut score_sum = 0usize;
    for z in 0..topographic_map.len() {
        for x in 0..topographic_map[0].len() {
            if topographic_map[z][x] != 0 {
                continue;
            }

            let mut next = VecDeque::new();
            next.push_back((x as i32, 0u8, z as i32));

            while !next.is_empty() {
                let (nx, ny, nz) = next.pop_front().unwrap();
                if ny == 9 {
                    score_sum += 1;
                    continue;
                }
                if nx - 1 >= 0 && topographic_map[nz as usize][(nx - 1) as usize] == ny + 1 {
                    next.push_back((nx - 1, ny + 1, nz));
                }
                if nx + 1 < topographic_map[0].len() as i32
                    && topographic_map[nz as usize][(nx + 1) as usize] == ny + 1
                {
                    next.push_back((nx + 1, ny + 1, nz));
                }
                if nz - 1 >= 0 && topographic_map[(nz - 1) as usize][nx as usize] == ny + 1 {
                    next.push_back((nx, ny + 1, nz - 1));
                }
                if nz + 1 < topographic_map.len() as i32
                    && topographic_map[(nz + 1) as usize][nx as usize] == ny + 1
                {
                    next.push_back((nx, ny + 1, nz + 1));
                }
            }
        }
    }

    score_sum
}
