pub mod part1;
pub mod part2;

fn map_input(input: String, map: &mut Vec<Vec<char>>) {
    input.lines().for_each(|row| {
        let mut row_map = Vec::new();
        row.chars().for_each(|col| row_map.push(col));
        map.push(row_map);
    });
}
