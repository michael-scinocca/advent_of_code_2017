use std::fmt::Display;

struct Buffer {
    buffer: Vec<u32>,
    current_pos: usize,
}

impl Buffer {
    fn new() -> Self {
        Buffer {
            buffer: Vec::new(),
            current_pos: 0,
        }
    }

    fn run(&mut self, steps: usize, iterations: usize, trace: bool) {
        self.buffer.push(0);

        if trace {
            println!("{}", self);
        }

        for value in 1..=iterations {
            self.increment_and_insert(steps, value);

            if trace {
                println!("{}", self);
            }
        }
    }

    fn increment_and_insert(&mut self, steps: usize, value: usize) {
        self.current_pos = (self.current_pos + steps) % self.buffer.len();
        self.current_pos += 1;

        self.buffer.insert(self.current_pos, value as u32);
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, value) in self.buffer.iter().enumerate() {
            if index == self.current_pos {
                f.write_str("(")?;
            }
            f.write_str(&value.to_string())?;
            if index == self.current_pos {
                f.write_str(")")?;
            }
            f.write_str(" ")?;
        }

        Ok(())
    }
}

pub fn part1() {
    let mut buffer = Buffer::new();
    buffer.run(304, 2017, false);

    println!("{}", buffer.buffer[buffer.current_pos + 1]);
}

struct BufferZero {
    length: usize,
    current_pos: usize,
    slot1: u32,
}

impl BufferZero {
    fn new() -> Self {
        BufferZero {
            length: 0,
            current_pos: 0,
            slot1: 0,
        }
    }

    fn run(&mut self, steps: usize, iterations: usize) {
        self.length += 1;

        for value in 1..=iterations {
            self.increment_and_insert(steps, value);
        }
    }

    fn increment_and_insert(&mut self, steps: usize, value: usize) {
        self.current_pos = (self.current_pos + steps) % self.length;
        self.current_pos += 1;

        if self.current_pos == 1 {
            self.slot1 = value as u32;
        }

        self.length += 1;
    }
}

pub fn part2() {
    let mut buffer = BufferZero::new();
    buffer.run(304, 5000000);

    println!("{}", buffer.slot1);
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(3, 638)]
    #[case(304, 1173)]
    fn test1(#[case] steps: usize, #[case] expected: u32) {
        let mut buffer = Buffer::new();
        buffer.run(steps, 2017, false);

        assert_eq!(expected, buffer.buffer[buffer.current_pos + 1]);
    }
}
