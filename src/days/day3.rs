use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

pub fn part2() {
    let value = find_value(265149);

    println!("{}", value);
}

fn find_value(goal: u32) -> u32 {
    let mut position = Position { x: 0, y: 0 };

    let mut square_size = 3;

    let mut map = HashMap::new();
    map.insert(position.clone(), 1);

    loop {
        if let Some(answer) = build_square_2(&mut map, &mut position, square_size, goal) {
            break answer;
        }

        square_size += 2;
    }
}

fn build_square_2(
    map: &mut HashMap<Position, u32>,
    position: &mut Position,
    square_size: u32,
    goal: u32,
) -> Option<u32> {
    // Kick right
    position.x += 1;
    let value = get_new_value(map, position);
    if value > goal {
        return Some(value);
    } else {
        map.insert(position.clone(), value);
    }

    // Right side up
    for _ in 0..square_size - 2 {
        position.y += 1;
        let value = get_new_value(map, position);
        if value > goal {
            return Some(value);
        } else {
            map.insert(position.clone(), value);
        }
    }

    // Top left
    for _ in 0..square_size - 1 {
        position.x -= 1;
        let value = get_new_value(map, position);
        if value > goal {
            return Some(value);
        } else {
            map.insert(position.clone(), value);
        }
    }

    // Left bottom
    for _ in 0..square_size - 1 {
        position.y -= 1;
        let value = get_new_value(map, position);
        if value > goal {
            return Some(value);
        } else {
            map.insert(position.clone(), value);
        }
    }

    // Bottom right
    for _ in 0..square_size - 1 {
        position.x += 1;
        let value = get_new_value(map, position);
        if value > goal {
            return Some(value);
        } else {
            map.insert(position.clone(), value);
        }
    }

    None
}

fn get_new_value(map: &HashMap<Position, u32>, position: &Position) -> u32 {
    let mut value = 0;

    if let Some(val) = map.get(&Position {
        x: position.x + 1,
        y: position.y,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x - 1,
        y: position.y,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x,
        y: position.y + 1,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x,
        y: position.y - 1,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x + 1,
        y: position.y + 1,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x + 1,
        y: position.y - 1,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x - 1,
        y: position.y + 1,
    }) {
        value += val;
    }

    if let Some(val) = map.get(&Position {
        x: position.x - 1,
        y: position.y - 1,
    }) {
        value += val;
    }

    value
}
