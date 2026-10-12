use std::collections::{HashSet, VecDeque};

use crate::days::day10;

pub fn part1() {
    let mut used = 0;

    for row in 0..128 {
        let mut data = (0..256).collect::<Vec<_>>();
        let hash = day10::get_hash(&mut data, &format!("jzgqcdpd-{}", row));

        for char in hash.chars() {
            for i in 0..4 {
                if (char.to_digit(16).unwrap() & (1 << i)) >> i == 1 {
                    used += 1;
                }
            }
        }
    }

    println!("{}", used);
}

#[derive(Copy, Clone)]
struct Cell {
    value: u8,
    group: u32,
}

impl Cell {
    fn new() -> Self {
        Cell { value: 0, group: 0 }
    }
}

pub fn part2() {
    let mut grid = [[Cell::new(); 128]; 128];

    for row in 0..128 {
        let mut data = (0..256).collect::<Vec<_>>();
        let hash = day10::get_hash(&mut data, &format!("jzgqcdpd-{}", row));

        for (index, char) in hash.char_indices() {
            for i in 0..4 {
                if (char.to_digit(16).unwrap() & (1 << i)) >> i == 1 {
                    grid[row as usize][index * 4 + 3 - i].value = 1;
                }
            }
        }
    }

    let mut group = 1;

    for row in 0..128 {
        for col in 0..128 {
            if grid[row][col].value != 1 {
                continue;
            }

            if grid[row][col].group != 0 {
                continue;
            }

            find_group(&mut grid, group, row, col);

            group += 1;
        }
    }

    println!("{}", group - 1);
}

fn find_group(grid: &mut [[Cell; 128]; 128], group: u32, row: usize, col: usize) {
    let mut moves = VecDeque::new();

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    if grid[row][col].value == 0 || grid[row][col].group != 0 {
        return;
    }

    grid[row][col].group = group;

    moves.push_back((row as isize - 1_isize, col as isize));
    moves.push_back((row as isize + 1_isize, col as isize));
    moves.push_back((row as isize, col as isize - 1_isize));
    moves.push_back((row as isize, col as isize + 1_isize));

    visited.insert((row as isize, col as isize));

    while let Some((next_row, next_col)) = moves.pop_front() {
        if !(0..128).contains(&next_row) || !(0..128).contains(&next_col) {
            continue;
        }

        if grid[next_row as usize][next_col as usize].value == 0 {
            continue;
        }

        if grid[next_row as usize][next_col as usize].group != 0 {
            continue;
        }

        if visited.contains(&(next_row, next_col)) {
            continue;
        }

        moves.push_back((next_row - 1_isize, next_col));
        moves.push_back((next_row + 1_isize, next_col));
        moves.push_back((next_row, next_col - 1_isize));
        moves.push_back((next_row, next_col + 1_isize));

        grid[next_row as usize][next_col as usize].group = group;

        visited.insert((next_row, next_col));
    }
}
