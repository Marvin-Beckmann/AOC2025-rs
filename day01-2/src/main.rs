use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_2(puzzle_input: String) -> f64 {
    let total_string: Vec<&str> = puzzle_input.lines().collect();

    let mut current = 50;
    let mut count: f64 = 0.0;
    total_string.iter().for_each(|x| {
        let value: i64 = (x[1..]).parse().unwrap();
        let nonzero = current != 0;
        if x.starts_with("L") {
            current -= value;
        } else {
            current += value;
        };
        let count_increase = if current > 0 {
            (current as f64 / 100.0).floor()
        } else if current <= 0 {
            if nonzero {
                -((current - 1) as f64 / 100.0).floor()
            } else {
                -((current - 1) as f64 / 100.0).floor() - 1.0
            }
        } else {
            0.0
        };
        count += count_increase;
        current = current.rem_euclid(100);
    });
    count
}
