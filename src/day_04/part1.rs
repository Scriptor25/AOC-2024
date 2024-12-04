use crate::day_04::map_input;

fn check_n(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y < 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y - 1][x];
    let ca = map[y - 2][x];
    let cs = map[y - 3][x];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_ne(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y < 3 || x >= w - 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y - 1][x + 1];
    let ca = map[y - 2][x + 2];
    let cs = map[y - 3][x + 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_e(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if x >= w - 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y][x + 1];
    let ca = map[y][x + 2];
    let cs = map[y][x + 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_se(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y >= h - 3 || x >= w - 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y + 1][x + 1];
    let ca = map[y + 2][x + 2];
    let cs = map[y + 3][x + 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_s(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y >= h - 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y + 1][x];
    let ca = map[y + 2][x];
    let cs = map[y + 3][x];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_sw(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y >= h - 3 || x < 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y + 1][x - 1];
    let ca = map[y + 2][x - 2];
    let cs = map[y + 3][x - 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_w(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if x < 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y][x - 1];
    let ca = map[y][x - 2];
    let cs = map[y][x - 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

fn check_nw(map: &Vec<Vec<char>>, w: usize, h: usize, x: usize, y: usize) -> bool {
    if y < 3 || x < 3 {
        return false;
    }
    let cx = map[y][x];
    let cm = map[y - 1][x - 1];
    let ca = map[y - 2][x - 2];
    let cs = map[y - 3][x - 3];
    cx == 'X' && cm == 'M' && ca == 'A' && cs == 'S'
}

pub fn part1(input: String) -> i32 {
    let mut map = Vec::new();
    map_input(input, &mut map);

    let width = map[0].len();
    let height = map.len();

    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let cell = map[y][x];
            if cell != 'X' {
                continue;
            }
            if check_n(&map, width, height, x, y) {
                count += 1;
            }
            if check_ne(&map, width, height, x, y) {
                count += 1;
            }
            if check_e(&map, width, height, x, y) {
                count += 1;
            }
            if check_se(&map, width, height, x, y) {
                count += 1;
            }
            if check_s(&map, width, height, x, y) {
                count += 1;
            }
            if check_sw(&map, width, height, x, y) {
                count += 1;
            }
            if check_w(&map, width, height, x, y) {
                count += 1;
            }
            if check_nw(&map, width, height, x, y) {
                count += 1;
            }
        }
    }

    count
}
