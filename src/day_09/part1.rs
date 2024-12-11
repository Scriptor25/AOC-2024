use std::collections::VecDeque;

pub fn part1(input: String) -> usize {
    let in_chars = input.chars().collect::<Vec<char>>();
    let mut file_indices = Vec::new();
    let mut idx = 0;

    let mut free_indices = VecDeque::new();

    for i in 0..in_chars.len() {
        let ic = in_chars[i] as i32;
        if i % 2 == 0 {
            let used = ic - 0x30;
            for _ in 0..used {
                file_indices.push(idx);
            }
            idx += 1;
        } else {
            let free = ic - 0x30;
            for _ in 0..free {
                free_indices.push_back(file_indices.len());
                file_indices.push(!0);
            }
        }
    }

    let mut end = 0usize;
    for i in (0..file_indices.len()).rev() {
        if free_indices.is_empty() {
            end = i + 1;
            break;
        }

        if file_indices[i] == !0 {
            free_indices.pop_back();
        } else {
            let free_idx = free_indices.pop_front().unwrap();
            file_indices.swap(free_idx, i);
        }
    }

    file_indices.drain(end..file_indices.len());

    let mut checksum = 0usize;
    for i in 0..file_indices.len() {
        let oc = file_indices[i];
        checksum += i * oc;
    }

    checksum
}
