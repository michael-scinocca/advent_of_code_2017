use std::{io::stdout, thread, time::Duration};

use crossterm::{ExecutableCommand, cursor, terminal};

pub fn part1() {
    let _test_input = r"
        |
        |  +--+
        A  |  C
    F---|----E|--+
        |  |  |  D
        +B-+  +--+
    ";

    let input = std::fs::read_to_string("data/day19.txt").unwrap();

    let (path, _) = run_path(&input);

    println!("{}", path);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day19.txt").unwrap();

    let (_, steps) = run_path(&input);

    println!("{}", steps);
}

fn clear_screen() {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
}

fn run_path(input: &str) -> (String, u32) {
    let mut path = String::new();
    let mut steps = 0;

    let max_length = input.lines().map(|l| l.len()).max().unwrap();

    let grid = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            format!("{:<width$}", l, width = max_length)
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let mut position: (usize, usize) = (0, grid[0].iter().position(|c| *c == '|').unwrap());
    let mut direction: (isize, isize) = (1, 0);

    let display_mode = false;

    loop {
        if display_mode {
            clear_screen();
        }

        position.0 = position.0.saturating_add_signed(direction.0);
        position.1 = position.1.saturating_add_signed(direction.1);

        steps += 1;

        if display_mode {
            for (r, row) in grid.iter().enumerate() {
                for (c, cell) in row.iter().enumerate() {
                    if r == position.0 && c == position.1 {
                        print!("*");
                    } else {
                        print!("{}", cell);
                    }
                }

                println!();
            }
        }

        let current_cell = grid[position.0][position.1];

        if current_cell == '+' {
            // Going up or down, look left or right
            if direction.0 != 0 {
                direction.0 = 0;
                if position.1 > 0
                    && grid[position.0][position.1 - 1] != ' '
                    && grid[position.0][position.1 - 1] != '|'
                {
                    direction.1 = -1;
                } else {
                    direction.1 = 1;
                }
            }
            // Going left or right, look up or down
            else if direction.1 != 0 {
                direction.1 = 0;
                if position.0 > 0
                    && grid[position.0 - 1][position.1] != ' '
                    && grid[position.0 - 1][position.1] != '-'
                {
                    direction.0 = -1;
                } else {
                    direction.0 = 1;
                }
            }
        } else if current_cell == ' '
            || direction.0 < 0 && position.0 == 0
            || direction.1 < 0 && position.1 == 0
            || direction.0 > 0 && position.0 == grid.len()
            || direction.1 > 0 && position.1 == grid[0].len()
        {
            break (path, steps);
        } else if current_cell != '|' && current_cell != '-' {
            path.push(current_cell);
        }

        if display_mode {
            thread::sleep(Duration::from_millis(100));
        }
    }
}
