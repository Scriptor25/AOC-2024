use crate::day_05::{check_pages, for_each_pages, get_middle_page};

pub fn part1(input: String) -> i32 {
    for_each_pages(input, |pages, orders| {
        if check_pages(pages, orders) {
            get_middle_page(pages)
        } else {
            0
        }
    })
}
