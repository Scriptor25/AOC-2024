use crate::day_06::{find_guard, get_in_front, inside_map, move_step, turn_right, Guard};
use std::collections::HashMap;

pub fn part1(input: String) -> i32 {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let height = map.len() as i32;
    let width = map[0].len() as i32;

    let mut guard = Guard { x: 0, y: 0, d: 0 };

    find_guard(&map, width, height, &mut guard);

    let mut path = HashMap::new();

    while inside_map(width, height, &guard) {
        let block = get_in_front(&map, width, height, &guard);
        if block == '#' {
            turn_right(&mut guard);
        }
        path.insert((guard.x, guard.y), true);
        move_step(&mut guard);
    }

    path.len() as i32
}
