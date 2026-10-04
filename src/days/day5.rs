pub fn part1() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();

    println!("{}", run_jumps(&input, |v| v + 1));
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day5.txt").unwrap();

    println!(
        "{}",
        run_jumps(&input, |v| if v >= 3 { v - 1 } else { v + 1 })
    );
}

fn run_jumps(input: &str, modify: fn(i32) -> i32) -> u32 {
    let mut position = 0_i32;
    let mut jumps = 0;

    let mut instructions = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    loop {
        if position < 0 || position as usize >= instructions.len() {
            break;
        }

        let new_position = position + instructions[position as usize];
        instructions[position as usize] = modify(instructions[position as usize]);
        position = new_position;

        jumps += 1;
    }

    jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = r"
            0
            3
            0
            1
            -3
        ";

        assert_eq!(5, run_jumps(input, |v| v + 1));
    }

    #[test]
    fn test2() {
        let input = r"
            0
            3
            0
            1
            -3
        ";

        assert_eq!(10, run_jumps(input, |v| if v >= 3 { v - 1 } else { v + 1 }));
    }
}
