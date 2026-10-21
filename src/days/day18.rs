use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Snd {
        reg: String,
    },
    Set {
        reg: String,
        reg_or_val: String,
    },
    Add {
        reg: String,
        reg_or_val: String,
    },
    Mul {
        reg: String,
        reg_or_val: String,
    },
    Mod {
        reg: String,
        reg_or_val: String,
    },
    Rcv {
        reg_or_val: String,
    },
    Jgz {
        check_reg_or_val: String,
        jmp_reg_or_val: String,
    },
}

struct Program {
    registers: HashMap<String, i64>,
    last_frequency: i64,
    program: Vec<Instruction>,
    program_counter: usize,
}

impl Program {
    fn new() -> Self {
        Program {
            registers: HashMap::new(),
            last_frequency: 0,
            program: Vec::new(),
            program_counter: 0,
        }
    }

    fn load(&mut self, program: &str) {
        let program_lines = program
            .trim()
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty());

        self.program = program_lines
            .filter_map(|l| {
                if let Some(next) = l.strip_prefix("snd ") {
                    return Some(Instruction::Snd {
                        reg: next.trim().to_string(),
                    });
                }

                if let Some(next) = l.strip_prefix("set ") {
                    let mut parts = next.split(' ');

                    return Some(Instruction::Set {
                        reg: parts.next().unwrap().to_string(),
                        reg_or_val: parts.next().unwrap().parse().unwrap(),
                    });
                }

                if let Some(next) = l.strip_prefix("add ") {
                    let mut parts = next.split(' ');

                    return Some(Instruction::Add {
                        reg: parts.next().unwrap().to_string(),
                        reg_or_val: parts.next().unwrap().parse().unwrap(),
                    });
                }

                if let Some(next) = l.strip_prefix("mul ") {
                    let mut parts = next.split(' ');

                    return Some(Instruction::Mul {
                        reg: parts.next().unwrap().to_string(),
                        reg_or_val: parts.next().unwrap().parse().unwrap(),
                    });
                }

                if let Some(next) = l.strip_prefix("mod ") {
                    let mut parts = next.split(' ');

                    return Some(Instruction::Mod {
                        reg: parts.next().unwrap().to_string(),
                        reg_or_val: parts.next().unwrap().parse().unwrap(),
                    });
                }

                if let Some(next) = l.strip_prefix("rcv ") {
                    return Some(Instruction::Rcv {
                        reg_or_val: next.trim().to_string(),
                    });
                }

                if let Some(next) = l.strip_prefix("jgz ") {
                    let mut parts = next.split(' ');

                    return Some(Instruction::Jgz {
                        check_reg_or_val: parts.next().unwrap().to_string(),
                        jmp_reg_or_val: parts.next().unwrap().parse().unwrap(),
                    });
                }

                None
            })
            .collect();
    }

    fn run(&mut self) -> i64 {
        self.program_counter = 0;

        loop {
            let continue_processing = match &self.program[self.program_counter] {
                Instruction::Snd { reg } => {
                    if let Some(val) = self.registers.get(reg) {
                        self.last_frequency = *val;
                    }

                    self.advance_program_counter(1)
                }
                Instruction::Set { reg, reg_or_val } => {
                    let value = self.get_reg_val_or_val(reg_or_val);

                    if let Some(val) = self.registers.get_mut(reg) {
                        *val = value;
                    } else {
                        self.registers.insert(reg.clone(), value);
                    }

                    self.advance_program_counter(1)
                }
                Instruction::Add { reg, reg_or_val } => {
                    let value = self.get_reg_val_or_val(reg_or_val);

                    if let Some(val) = self.registers.get_mut(reg) {
                        *val += value;
                    } else {
                        self.registers.insert(reg.clone(), value);
                    }

                    self.advance_program_counter(1)
                }
                Instruction::Mul { reg, reg_or_val } => {
                    let value = self.get_reg_val_or_val(reg_or_val);

                    if let Some(val) = self.registers.get_mut(reg) {
                        *val *= value;
                    } else {
                        self.registers.insert(reg.clone(), 0);
                    }

                    self.advance_program_counter(1)
                }
                Instruction::Mod { reg, reg_or_val } => {
                    let value = self.get_reg_val_or_val(reg_or_val);

                    if let Some(val) = self.registers.get_mut(reg) {
                        *val %= value;
                    } else {
                        self.registers.insert(reg.clone(), 0);
                    }

                    self.advance_program_counter(1)
                }
                Instruction::Rcv { reg_or_val } => {
                    let value = self.get_reg_val_or_val(reg_or_val);

                    if value != 0 {
                        break self.last_frequency;
                    } else {
                        self.advance_program_counter(1)
                    }
                }
                Instruction::Jgz {
                    check_reg_or_val,
                    jmp_reg_or_val,
                } => {
                    let check_value = self.get_reg_val_or_val(check_reg_or_val);

                    if check_value > 0 {
                        let jmp_value = self.get_reg_val_or_val(jmp_reg_or_val);

                        self.advance_program_counter(jmp_value as isize)
                    } else {
                        self.advance_program_counter(1)
                    }
                }
            };

            if !continue_processing {
                break 0;
            }
        }
    }

    fn advance_program_counter(&mut self, step: isize) -> bool {
        let (new_val, overflowed) = self.program_counter.overflowing_add_signed(step);

        if overflowed {
            return false;
        }

        self.program_counter = new_val;

        self.program_counter < self.program.len()
    }

    fn get_reg_val_or_val(&self, reg_or_val: &str) -> i64 {
        reg_or_val
            .parse()
            .unwrap_or_else(|_| *self.registers.get(reg_or_val).unwrap_or(&0))
    }
}

pub fn part1() {
    let program_text = std::fs::read_to_string("data/day18.txt").unwrap();

    let mut program = Program::new();
    program.load(&program_text);

    let result = program.run();

    println!("{}", result);
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut program = Program::new();
        program.load(
            r"
            set a 1
            add a 2
            mul a a
            mod a 5
            snd a
            set a 0
            rcv a
            jgz a -1
            set a 1
            jgz a -2
        ",
        );

        let result = program.run();

        assert_eq!(4, result);
    }
}
