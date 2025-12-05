use std::fs;

fn main() {
    let puzzle_input = fs::read_to_string("puzzle_input.txt").unwrap();
    println!("{}", solve_2(puzzle_input));
}

pub fn solve_1(puzzle_input: String) -> i64 {
    let total_string: Vec<&str> = puzzle_input.lines().collect();
    let mut count = 0;

    let max_row = (total_string.len() - 1) as i32;
    let max_column = (total_string[0].len() - 1) as i32;

    for row in 0..=max_row {
        for column in 0..=max_column {
            if total_string[row as usize]
                .chars()
                .nth(column as usize)
                .unwrap()
                == '@'
            {
                let neigbhours = get_neigbours(row, column, max_row, max_column);
                if neigbhours
                    .iter()
                    .map(|(x, y)| {
                        let (x, y) = (*x as usize, *y as usize);
                        total_string[x].chars().nth(y).unwrap()
                    })
                    .filter(|x| x == &'@')
                    .count()
                    < 4
                {
                    count += 1
                };
            }
        }
    }

    count
}

fn get_neigbours(row: i32, column: i32, max_row: i32, max_column: i32) -> Vec<(i32, i32)> {
    let mut neighbours = Vec::new();
    for (x, y) in [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ] {
        neighbours.push((row + x, column + y));
    }
    neighbours
        .iter()
        .filter(|(x, y)| x >= &0 && x <= &max_row && y >= &0 && y <= &max_column)
        .map(|(x, y)| (*x, *y))
        .collect()
}

pub fn solve_2(puzzle_input: String) -> i32 {
    let mut total_string: Vec<Vec<char>> =
        puzzle_input.lines().map(|x| x.chars().collect()).collect();
    let mut count = 0;

    let max_row = (total_string.len() - 1) as i32;
    let max_column = (total_string[0].len() - 1) as i32;
    let mut remove_nodes = true;
    let mut nodes_to_remove: Vec<(usize, usize)> = Vec::new();
    while remove_nodes {
        for (x, y) in nodes_to_remove {
            total_string[x][y] = '.';
            count += 1
        }
        nodes_to_remove = Vec::new();
        for row in 0..=max_row {
            for column in 0..=max_column {
                if total_string[row as usize][column as usize] == '@' {
                    let neigbhours = get_neigbours(row, column, max_row, max_column);
                    if neigbhours
                        .iter()
                        .map(|(x, y)| {
                            let (x, y) = (*x as usize, *y as usize);
                            total_string[x][y]
                        })
                        .filter(|x| x == &'@')
                        .count()
                        < 4
                    {
                        nodes_to_remove.push((row as usize, column as usize));
                    };
                }
            }
        }
        if nodes_to_remove.is_empty() {
            remove_nodes = false
        }
    }

    count
}
