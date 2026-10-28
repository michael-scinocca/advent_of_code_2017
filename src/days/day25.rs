use std::collections::HashMap;

enum State {
    A,
    B,
    C,
    D,
    E,
    F,
}

struct TapeDrive {
    tape: HashMap<i32, u32>,
    position: i32,
}

impl TapeDrive {
    fn new() -> Self {
        TapeDrive {
            tape: HashMap::new(),
            position: 0,
        }
    }

    fn read(&self) -> u32 {
        *self.tape.get(&self.position).unwrap_or(&0)
    }

    fn write(&mut self, value: u32) {
        self.tape
            .entry(self.position)
            .and_modify(|v| *v = value)
            .or_insert(value);
    }

    fn move_position(&mut self, delta: i32) {
        self.position += delta;
    }

    fn checksum(&self) -> usize {
        self.tape.values().filter(|v| **v == 1).count()
    }
}

pub fn part1() {
    let mut tape_drive = TapeDrive::new();

    let mut current_state = State::A;

    for _ in 0..12386363 {
        current_state = operate(current_state, &mut tape_drive);
    }

    println!("{}", tape_drive.checksum());
}

pub fn part2() {}

fn operate(state: State, tape_drive: &mut TapeDrive) -> State {
    let current_value = tape_drive.read();

    match state {
        State::A => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(1);
                State::B
            } else {
                tape_drive.write(0);
                tape_drive.move_position(-1);
                State::E
            }
        }
        State::B => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::C
            } else {
                tape_drive.write(0);
                tape_drive.move_position(1);
                State::A
            }
        }
        State::C => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::D
            } else {
                tape_drive.write(0);
                tape_drive.move_position(1);
                State::C
            }
        }
        State::D => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::E
            } else {
                tape_drive.write(0);
                tape_drive.move_position(-1);
                State::F
            }
        }
        State::E => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::A
            } else {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::C
            }
        }
        State::F => {
            if current_value == 0 {
                tape_drive.write(1);
                tape_drive.move_position(-1);
                State::E
            } else {
                tape_drive.write(1);
                tape_drive.move_position(1);
                State::A
            }
        }
    }
}
