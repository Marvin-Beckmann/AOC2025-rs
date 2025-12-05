use std::{cmp::max, fs};

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let (ranges, ids) = puzzle_input.split_once("\n\n").unwrap();
    let upper_lower: Vec<(usize, usize)> = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut count = 0;
    for id in ids.lines() {
        let id_int: usize = id.parse().unwrap();
        if upper_lower
            .iter()
            .any(|(lower, upper)| &id_int >= lower && &id_int <= upper)
        {
            count += 1
        }
    }

    count
}

pub fn solve_2(puzzle_input: String) -> usize {
    let (ranges, _) = puzzle_input.split_once("\n\n").unwrap();
    let mut upper_lower: Vec<(usize, usize)> = ranges
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut count = 0;
    let mut cur_end_id = 0;

    upper_lower.sort_by(|(x_0, _), (x_1, _)| x_0.cmp(x_1));

    for i in 0..upper_lower.len() {
        if i == 0 {
            cur_end_id = upper_lower[i].1;
            count += upper_lower[i].1 - upper_lower[i].0 + 1
        } else {
            let next_start = max(upper_lower[i].0, cur_end_id + 1);
            if upper_lower[i].1 >= next_start {
                count += upper_lower[i].1 - next_start + 1;
                cur_end_id = upper_lower[i].1
            }
        }
    }

    count
}
