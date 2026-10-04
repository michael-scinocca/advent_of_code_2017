#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

pub fn part1() {
    let position = find_position(265149);

    println!("{}", position.x.abs() + position.y.abs());
}

fn find_position(goal: u32) -> Position {
    let mut position = Position { x: 0, y: 0 };
    let mut index = 1;

    let mut square_size = 3;

    loop {
        if build_square(&mut position, &mut index, square_size, goal) {
            break position;
        }

        square_size += 2;
    }
}

fn build_square(position: &mut Position, index: &mut u32, square_size: u32, goal: u32) -> bool {
    // Kick right
    position.x += 1;
    *index += 1;

    if *index == goal {
        return true;
    }

    // Right side up
    for _ in 0..square_size - 2 {
        position.y += 1;
        *index += 1;

        if *index == goal {
            return true;
        }
    }

    // Top left
    for _ in 0..square_size - 1 {
        position.x -= 1;
        *index += 1;

        if *index == goal {
            return true;
        }
    }

    // Left bottom
    for _ in 0..square_size - 1 {
        position.y -= 1;
        *index += 1;

        if *index == goal {
            return true;
        }
    }

    // Bottom right
    for _ in 0..square_size - 1 {
        position.x += 1;
        *index += 1;

        if *index == goal {
            return true;
        }
    }

    false
}

pub fn part2() {}
