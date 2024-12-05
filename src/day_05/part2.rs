use crate::day_05::{check_pages, for_each_pages, get_middle_page, is_ordered};
use std::cmp::Ordering;

fn order_pages(pages: &mut Vec<i32>, orders: &Vec<(i32, i32)>) {
    pages.sort_by(|&a, &b| {
        if is_ordered(orders, (a, b)) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
}

pub fn part2(input: String) -> i32 {
    for_each_pages(input, |pages, orders| {
        if !check_pages(pages, orders) {
            order_pages(pages, orders);
            get_middle_page(pages)
        } else {
            0
        }
    })
}
