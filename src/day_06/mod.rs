pub mod part1;
pub mod part2;

struct Guard {
    x: i32,
    y: i32,
    d: i32,
}

fn pos_inside_map(width: i32, height: i32, x: i32, y: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn inside_map(width: i32, height: i32, guard: &Guard) -> bool {
    pos_inside_map(width, height, guard.x, guard.y)
}

fn map_at(map: &Vec<Vec<char>>, width: i32, height: i32, x: i32, y: i32) -> char {
    if pos_inside_map(width, height, x, y) {
        map[y as usize][x as usize]
    } else {
        '~'
    }
}

fn find_guard(map: &Vec<Vec<char>>, width: i32, height: i32, guard: &mut Guard) {
    for j in 0..height {
        for i in 0..width {
            let col = map_at(map, width, height, i, j);
            if col == '^' {
                guard.x = i;
                guard.y = j;
                return;
            }
        }
    }
}

fn get_in_front(map: &Vec<Vec<char>>, width: i32, height: i32, guard: &Guard) -> char {
    match guard.d {
        0 => map_at(map, width, height, guard.x, guard.y - 1),
        1 => map_at(map, width, height, guard.x + 1, guard.y),
        2 => map_at(map, width, height, guard.x, guard.y + 1),
        3 => map_at(map, width, height, guard.x - 1, guard.y),
        _ => '~',
    }
}

fn turn_right(guard: &mut Guard) {
    guard.d = (guard.d + 1) % 4
}

fn move_step(guard: &mut Guard) {
    match guard.d {
        0 => {
            guard.y -= 1;
        }
        1 => {
            guard.x += 1;
        }
        2 => {
            guard.y += 1;
        }
        3 => {
            guard.x -= 1;
        }
        _ => {}
    };
}
