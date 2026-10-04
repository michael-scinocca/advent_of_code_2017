use std::collections::HashMap;

pub fn part1() {
    let mut input = std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .split_whitespace()
        .filter(|i| !i.trim().is_empty())
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", get_runs(&mut input).0);
}

fn get_runs(input: &mut [i32]) -> (u32, usize) {
    let mut current_index = 0;

    let mut combinations = HashMap::new();
    combinations.insert(input.to_owned(), current_index);

    let mut runs = 0;

    loop {
        runs += 1;

        let highest_val = *input.iter().max().unwrap();
        let index = input.iter().position(|v| *v == highest_val).unwrap();
        let mut next_index = index;

        while input[index] > 0 {
            next_index += 1;

            if next_index >= input.len() {
                next_index -= input.len();
            }

            input[index] -= 1;
            input[next_index] += 1;
        }

        if combinations.contains_key(input) {
            let index = combinations.get(input).unwrap();
            break (runs, runs as usize - index);
        }

        current_index += 1;
        combinations.insert(input.to_owned(), current_index);
    }
}

pub fn part2() {
    let mut input = std::fs::read_to_string("data/day6.txt")
        .unwrap()
        .split_whitespace()
        .filter(|i| !i.trim().is_empty())
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", get_runs(&mut input).1);
}
