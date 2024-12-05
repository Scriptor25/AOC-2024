pub mod part1;
pub mod part2;

fn is_ordered(orders: &Vec<(i32, i32)>, pair: (i32, i32)) -> bool {
    let order = orders
        .iter()
        .find(|p| (p.0 == pair.0 && p.1 == pair.1) || (p.1 == pair.0 && p.0 == pair.1));
    if order.is_none() {
        return true;
    }
    let o = order.unwrap();
    if o.0 == pair.0 && o.1 == pair.1 {
        true
    } else {
        false
    }
}

fn check_pages(pages: &Vec<i32>, orders: &Vec<(i32, i32)>) -> bool {
    pages.is_sorted_by(|&a, &b| is_ordered(orders, (a, b)))
}

fn get_middle_page(pages: &Vec<i32>) -> i32 {
    pages[pages.len() / 2]
}

fn for_each_pages(input: String, consumer: fn(&mut Vec<i32>, &Vec<(i32, i32)>) -> i32) -> i32 {
    let mut orders = Vec::<(i32, i32)>::new();
    let mut sum = 0;

    input.lines().for_each(|line| {
        if line.len() == 5 {
            let fst: i32 = line.split_at(2).0.parse().unwrap();
            let sec: i32 = line.split_at(3).1.parse().unwrap();
            orders.push((fst, sec));
            return;
        }

        let mut pages = line
            .split(",")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        if pages.is_empty() {
            return;
        }

        sum += consumer(&mut pages, &orders);
    });

    sum
}
