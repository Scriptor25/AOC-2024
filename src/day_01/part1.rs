use crate::day_01::split_columns;

pub fn part1(input: String) -> i32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    split_columns(input, &mut list1, &mut list2);

    list1.sort_unstable();
    list2.sort_unstable();

    let mut sum = 0;
    for i in 0..list1.len() {
        let num1 = list1[i];
        let num2 = list2[i];
        sum += (num1 - num2).abs();
    }

    sum
}
