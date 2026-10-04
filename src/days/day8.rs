use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();

    let (highest, _) = run_program(&input);

    println!("{}", highest);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day8.txt").unwrap();

    let (_, highest) = run_program(&input);

    println!("{}", highest);
}

fn run_program(input: &str) -> (i64, i64) {
    let mut registers = HashMap::new();

    let mut highest = 0_i64;

    for line in input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim())
    {
        let mut parts = line.split_whitespace();

        let register_id = parts.next().unwrap().trim();

        let operation = parts.next().unwrap().trim();
        let op_value = parts.next().unwrap().trim().parse::<i64>().unwrap();

        let check_register = parts.nth(1).unwrap().trim();
        let check_condition = parts.next().unwrap().trim();
        let check_val = parts.next().unwrap().trim().parse::<i64>().unwrap();

        let check_reg_value = registers.entry(check_register).or_insert(0_i64);

        let check_passed = match check_condition {
            ">" => *check_reg_value > check_val,
            ">=" => *check_reg_value >= check_val,
            "<" => *check_reg_value < check_val,
            "<=" => *check_reg_value <= check_val,
            "==" => *check_reg_value == check_val,
            "!=" => *check_reg_value != check_val,
            other => panic!("Unknown condition {other}"),
        };

        if !check_passed {
            continue;
        }

        let offset = match operation {
            "inc" => op_value,
            "dec" => -op_value,
            other => panic!("Unknown operation {other}"),
        };

        registers
            .entry(register_id)
            .and_modify(|r| {
                *r += offset;
            })
            .or_insert(offset);

        highest = highest.max(*registers.values().max().unwrap());
    }

    (*registers.values().max().unwrap(), highest)
}
