use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_1(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let total_string = puzzle_input.lines().next().unwrap();
    let dashed_ids: Vec<&str> = total_string.split(",").collect();

    let ids: Vec<(&str, &str)> = dashed_ids
        .iter()
        .map(|x| x.split_once("-").unwrap())
        .collect();
    let mut count = 0;

    for (id1, id2) in ids {
        let lower_id: i64 = id1.parse().unwrap();
        let upper_id: i64 = id2.parse().unwrap();

        let iterations = upper_id - lower_id;
        for iteration in 0..=iterations {
            let id = lower_id + iteration;
            let id_string = id.to_string();
            let (lh, rh) = id_string.split_at(id_string.len() / 2);
            if lh == rh {
                count += id;
            }
        }
    }

    count
}
