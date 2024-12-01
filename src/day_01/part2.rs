use crate::day_01::split_columns;

pub fn part2(input: String) -> i32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    split_columns(input, &mut list1, &mut list2);

    let mut similarity = 0;

    for li in list1 {
        let count = list2.iter().filter(|&&x| x == li).count();
        similarity += li * count as i32;
    }

    similarity
}
