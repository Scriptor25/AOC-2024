use std::collections::HashMap;

#[derive(PartialEq)]
struct Node {
    x: i32,
    y: i32,
}

fn antenna_vec(a: &Node, b: &Node) -> (i32, i32) {
    (b.x - a.x, b.y - a.y)
}

fn extend(a: &Node, b: &Node, w: i32, h: i32) -> Vec<Node> {
    let vec = antenna_vec(a, b);

    let mut result = Vec::new();

    let mut pos = (a.x, a.y);
    while inside(&pos, w, h) {
        result.push(Node { x: pos.0, y: pos.1 });
        pos.0 -= vec.0;
        pos.1 -= vec.1;
    }

    pos = (b.x, b.y);
    while inside(&pos, w, h) {
        result.push(Node { x: pos.0, y: pos.1 });
        pos.0 += vec.0;
        pos.1 += vec.1;
    }

    result
}

fn inside(pos: &(i32, i32), w: i32, h: i32) -> bool {
    pos.0 >= 0 && pos.0 < w && pos.1 >= 0 && pos.1 < h
}

fn is_unique(node: &Node, map: &Vec<Vec<char>>) -> bool {
    map[node.y as usize][node.x as usize] != '#'
}

pub fn part2(input: String) -> usize {
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

    for (_, nodes) in node_map {
        for i in &nodes {
            for j in &nodes {
                if i == j {
                    continue;
                }

                let anti_nodes = extend(i, j, width, height);
                for anti_node in anti_nodes {
                    if is_unique(&anti_node, &map) {
                        count += 1;
                        map[anti_node.y as usize][anti_node.x as usize] = '#';
                    }
                }
            }
        }
    }

    map.iter().for_each(|row| {
        println!("{}", row.iter().collect::<String>());
    });

    count as usize
}
