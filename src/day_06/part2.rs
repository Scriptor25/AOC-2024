use crate::day_06::{find_guard, get_in_front, inside_map, map_at, move_step, turn_right, Guard};
use std::collections::HashSet;
use std::io::{stdout, Write};

pub fn part2(input: String) -> i32 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut guard = Guard { x: 0, y: 0, d: 0 };

    find_guard(&map, width, height, &mut guard);

    let mut loop_count = 0;

    for y in 0..height {
        let p = (100.0 * y as f32 / height as f32) as i32;
        print!("\r{:3}%", p);
        stdout().flush();
        for x in 0..width {
            if (x == guard.x && y == guard.y) || map_at(&map, width, height, x, y) != '.' {
                continue;
            }

            let mut test_map = map.clone();
            test_map[y as usize][x as usize] = 'O';

            if creates_loop(&test_map, width, height, &guard) {
                loop_count += 1;
            }
        }
    }
    println!("\rDone.");

    loop_count
}

fn creates_loop(map: &Vec<Vec<char>>, width: i32, height: i32, guard: &Guard) -> bool {
    let mut visited_states = HashSet::new();
    let mut guard = guard.clone();

    while inside_map(width, height, &guard) {
        let state = (guard.x, guard.y, guard.d);

        if visited_states.contains(&state) {
            return true;
        }

        visited_states.insert(state);

        let block = get_in_front(&map, width, height, &guard);
        if block == '#' || block == 'O' {
            turn_right(&mut guard);
        } else {
            move_step(&mut guard);
        }
    }

    false
}
