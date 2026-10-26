use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("data/day21.txt").unwrap();

    let on_cells = run_enhancement(&input, 5);

    println!("{}", on_cells);
}

pub fn part2() {
    let input = std::fs::read_to_string("data/day21.txt").unwrap();

    let on_cells = run_enhancement(&input, 18);

    println!("{}", on_cells);
}

fn run_enhancement(rules: &str, iterations: u32) -> usize {
    let mut grid = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];

    print_grid(&grid);

    let transforms = process_rules(rules);

    for _ in 0..iterations {
        let mut next_grid = Vec::new();

        let mut pos: (usize, usize, usize) = (0, 0, 0);

        loop {
            let grid_square = {
                if grid.len() % 2 == 0 {
                    vec![
                        vec![grid[pos.0][pos.1], grid[pos.0][pos.1 + 1]],
                        vec![grid[pos.0 + 1][pos.1], grid[pos.0 + 1][pos.1 + 1]],
                    ]
                } else {
                    vec![
                        vec![
                            grid[pos.0][pos.1],
                            grid[pos.0][pos.1 + 1],
                            grid[pos.0][pos.1 + 2],
                        ],
                        vec![
                            grid[pos.0 + 1][pos.1],
                            grid[pos.0 + 1][pos.1 + 1],
                            grid[pos.0 + 1][pos.1 + 2],
                        ],
                        vec![
                            grid[pos.0 + 2][pos.1],
                            grid[pos.0 + 2][pos.1 + 1],
                            grid[pos.0 + 2][pos.1 + 2],
                        ],
                    ]
                }
            };

            let next_grid_square = transform_grid(&grid_square, &transforms);

            for row in 0..next_grid_square.len() {
                let next_grid_row_index = row + pos.2 * next_grid_square.len();

                if next_grid.len() <= next_grid_row_index {
                    next_grid.push(Vec::new());
                }

                next_grid[next_grid_row_index].extend(next_grid_square[row].clone());
            }

            pos.0 += grid_square.len();
            pos.2 += 1;

            if pos.0 > grid.len() - grid_square.len() {
                pos.0 = 0;
                pos.2 = 0;
                pos.1 += grid_square.len();
            }

            if pos.1 > grid.len() - grid_square.len() {
                grid = next_grid.clone();
                break;
            }
        }

        print_grid(&next_grid);
    }

    grid.iter().flatten().filter(|c| **c == 1).count()
}

fn transform_grid(
    grid: &[Vec<i32>],
    transforms: &HashMap<Vec<Vec<i32>>, Vec<Vec<i32>>>,
) -> Vec<Vec<i32>> {
    if let Some(transform) = transforms.get(grid) {
        transform.clone()
    } else {
        print_grid(grid);
        panic!("No transform found");
    }
}

fn process_rules(rules: &str) -> HashMap<Vec<Vec<i32>>, Vec<Vec<i32>>> {
    let mut transforms = HashMap::new();

    for line in rules.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let mut parts = line.split("=>");

        let input = parts.next().unwrap().trim();
        let output = parts.next().unwrap().trim();

        let input_parts = input.split('/');

        let mut input = Vec::new();

        for input_part in input_parts {
            input.push(
                input_part
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect(),
            );
        }

        let inputs = get_all_aliases(input);

        let output_parts = output.split('/');

        let mut output = Vec::new();

        for output_part in output_parts {
            output.push(
                output_part
                    .chars()
                    .map(|c| if c == '#' { 1 } else { 0 })
                    .collect(),
            );
        }

        for input in inputs {
            transforms.insert(input, output.clone());
        }
    }

    transforms
}

fn get_all_aliases(grid: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut aliases = Vec::new();

    let mut grid = grid.clone();
    let mut flipped = flip_x(&grid);

    aliases.push(grid.clone());

    for _ in 0..3 {
        grid = rotate(&grid);
        aliases.push(grid.clone());
    }

    aliases.push(flipped.clone());

    for _ in 0..3 {
        flipped = rotate(&flipped);
        aliases.push(flipped.clone());
    }

    aliases
}

fn flip_x(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut grid = grid.to_owned();

    for row in &mut grid {
        row.reverse();
    }

    grid
}

fn rotate(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut grid = grid.to_owned();

    if grid.len() == 2 {
        let first = grid[0][0];
        grid[0][0] = grid[1][0];
        grid[1][0] = grid[1][1];
        grid[1][1] = grid[0][1];
        grid[0][1] = first;
    } else if grid.len() == 3 {
        let first = grid[0][1];
        grid[0][1] = grid[1][0];
        grid[1][0] = grid[2][1];
        grid[2][1] = grid[1][2];
        grid[1][2] = first;

        let first = grid[0][0];
        grid[0][0] = grid[2][0];
        grid[2][0] = grid[2][2];
        grid[2][2] = grid[0][2];
        grid[0][2] = first;
    }

    grid
}

fn print_grid(grid: &[Vec<i32>]) {
    for row in grid {
        for cell in row {
            print!("{}", if *cell == 1 { '#' } else { '.' });
        }

        println!();
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let on_cells = run_enhancement(
            r"
            ../.# => ##./#../...
            .#./..#/### => #..#/..../..../#..#
            ",
            2,
        );

        assert_eq!(12, on_cells);
    }
}
