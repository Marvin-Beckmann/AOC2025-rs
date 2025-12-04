use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let total_string: Vec<&str> = puzzle_input.lines().collect();
    let mut count = 0;

    for line in total_string {
        let max = line[..(line.len() - 1)]
            .chars()
            .max_by(|x, y| {
                let (xval, yval) = (x.to_digit(10).unwrap(), y.to_digit(10).unwrap());
                xval.cmp(&yval)
            })
            .unwrap();
        let max_first_pos = line.find(max).unwrap();
        let second_max = line[(1 + max_first_pos)..]
            .chars()
            .max_by(|x, y| {
                let (xval, yval) = (x.to_digit(10).unwrap(), y.to_digit(10).unwrap());
                xval.cmp(&yval)
            })
            .unwrap();
        let power = format!("{max}{second_max}");
        println!("battery: {line}\npower: {power}");
        let battery_level: i64 = power.parse().unwrap();
        count += battery_level
    }

    count
}

pub fn solve_2(puzzle_input: String) -> i64 {
    let total_string: Vec<&str> = puzzle_input.lines().collect();
    let mut count = 0;

    for line in total_string {
        let mut battery = "".to_string();
        let mut current_start = 0;
        for i in 0..12 {
            let next = line[(current_start)..(line.len() - 11 + i)]
                .chars()
                .max_by(|x, y| {
                    let (xval, yval) = (x.to_digit(10).unwrap(), y.to_digit(10).unwrap());
                    xval.cmp(&yval)
                })
                .unwrap();
            current_start = current_start + line[(current_start)..].find(next).unwrap() + 1;
            battery = format!("{}{next}", &battery);
        }
        let battery_level: i64 = battery.parse().unwrap();
        count += battery_level
    }

    count
}
