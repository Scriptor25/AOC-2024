use std::collections::HashMap;

#[derive(PartialEq)]
struct Node {
    x: i32,
    y: i32,
}

fn antenna_vec(a: &Node, b: &Node) -> (i32, i32) {
    (b.x - a.x, b.y - a.y)
}

fn extend(a: &Node, b: &Node) -> (Node, Node) {
    let vec = antenna_vec(a, b);
    let a_a = Node {
        x: a.x - vec.0,
        y: a.y - vec.1,
    };
    let a_b = Node {
        x: b.x + vec.0,
        y: b.y + vec.1,
    };
    (a_a, a_b)
}

fn inside(node: &Node, w: i32, h: i32) -> bool {
    node.x >= 0 && node.x < w && node.y >= 0 && node.y < h
}

fn is_unique(node: &Node, map: &Vec<Vec<char>>) -> bool {
    map[node.y as usize][node.x as usize] != '#'
}

pub fn part1(input: String) -> usize {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = map[0].len() as i32;
    let height = map.len() as i32;

    let mut node_map: HashMap<char, Vec<Node>> = HashMap::new();

    for row_index in 0..height {
        for col_index in 0..width {
            let freq = map[row_index as usize][col_index as usize];
            if freq != '.' {
                let node = Node {
                    x: col_index as i32,
                    y: row_index as i32,
                };
                if let Some(nodes) = node_map.get_mut(&freq) {
                    nodes.push(node);
                } else {
                    let mut nodes = Vec::new();
                    nodes.push(node);
                    node_map.insert(freq, nodes);
                }
            }
        }
    }

    let mut count = 0;

    for (freq, nodes) in node_map {
        for i in &nodes {
            for j in &nodes {
                if i == j {
                    continue;
                }

                let (anti_a, anti_b) = extend(i, j);
                if inside(&anti_a, width, height) && is_unique(&anti_a, &map) {
                    count += 1;
                    map[anti_a.y as usize][anti_a.x as usize] = '#';
                }
                if inside(&anti_b, width, height) && is_unique(&anti_b, &map) {
                    count += 1;
                    map[anti_b.y as usize][anti_b.x as usize] = '#';
                }
            }
        }
    }

    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });

    count as usize
}
