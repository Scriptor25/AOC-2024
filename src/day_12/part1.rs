fn find_cells(dst: &mut Vec<(usize, usize)>, map: &mut Vec<Vec<char>>, i: usize, j: usize) {
    
}

pub fn part1(input: String) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut sizes: Vec<(usize, usize)> = Vec::new();

    for j in 0..map.len() {
        for i in 0..map[0].len() {
            if map[j][i] == '#' {
                continue;
            }
            let mut cells = Vec::new();
            find_cells(&mut cells, &mut map, i, j);
        }
    }

    let mut sum = 0;
    for size in sizes {
        sum += size.0 * size.1
    }
    sum
}
