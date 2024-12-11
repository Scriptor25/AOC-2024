pub fn part2(input: String) -> usize {
    let in_chars = input.chars().collect::<Vec<char>>();

    let mut file_blocks = Vec::new();

    let mut idx = 0usize;
    for i in 0..in_chars.len() {
        let ic = in_chars[i] as i32;
        let size = (ic - 0x30) as usize;
        if size == 0 {
            continue;
        }
        if i % 2 == 0 {
            file_blocks.push((idx, size));
            idx += 1;
        } else {
            file_blocks.push((!0, size));
        }
    }

    {
        let mut i = file_blocks.len();
        while i > 0 {
            i -= 1;

            let (file_idx, file_size) = file_blocks[i];
            if file_idx == !0 {
                continue;
            }

            for j in 0..i {
                let (free_idx, free_size) = file_blocks[j];
                if free_idx != !0 {
                    continue;
                }
                if free_size == file_size {
                    file_blocks.swap(i, j);
                    break;
                }
                if free_size > file_size {
                    file_blocks[j].1 = free_size - file_size;
                    file_blocks[i] = (!0, file_size);
                    file_blocks.insert(j, (file_idx, file_size));
                    i += 1;
                    break;
                }
            }
        }
    }

    let mut checksum = 0usize;
    let mut idx = 0usize;
    for i in 0..file_blocks.len() {
        let (file_idx, file_size) = file_blocks[i];
        if file_idx != !0 {
            for j in 0..file_size {
                checksum += (idx + j) * file_idx;
            }
        }
        idx += file_size;
    }

    checksum
}
