use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let total_string: Vec<&str> = puzzle_input.lines().collect();

    let mut current = 50;
    let mut count = 0;
    total_string.iter().for_each(|x| {
        let value: i32 = (x[1..]).parse().unwrap();
        if x.starts_with("L") {
            current -= value;
        } else {
            current += value;
        };
        current %= 100;
        if current == 0 {
            count += 1
        }
    });
    count
}
