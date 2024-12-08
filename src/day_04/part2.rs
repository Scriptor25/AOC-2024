use crate::day_04::map_input;

pub fn part2(input: String) -> usize {
    let mut map = Vec::new();
    map_input(input, &mut map);

    let width = map[0].len();
    let height = map.len();

    let mut count = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let cell = map[y][x];
            if cell != 'A' {
                continue;
            }
            let c00 = map[y - 1][x - 1];
            let c01 = map[y - 1][x + 1];
            let c10 = map[y + 1][x - 1];
            let c11 = map[y + 1][x + 1];
            let d0 = (c00 == 'M' && c11 == 'S') || (c00 == 'S' && c11 == 'M');
            let d1 = (c01 == 'M' && c10 == 'S') || (c01 == 'S' && c10 == 'M');
            if d0 && d1 {
                count += 1;
            }
        }
    }

    count
}
