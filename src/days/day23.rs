use std::collections::HashMap;

#[derive(Debug)]
enum Instruction {
    Set {
        reg: String,
        reg_or_val: String,
    },
    Sub {
        reg: String,
        reg_or_val: String,
    },
    Mul {
        reg: String,
        reg_or_val: String,
    },
    Jnz {
        check_reg_or_val: String,
        jmp_reg_or_val: String,
    },
}

struct Program {
    registers: HashMap<String, i64>,
    program: Vec<Instruction>,
    program_counter: usize,
    mul_count: i64,
}

impl Program {
    fn new() -> Self {
        Self {
            registers: HashMap::new(),
            program: Vec::new(),
            program_counter: 0,
            mul_count: 0,
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
                let mut parts = l.split_whitespace();
                match parts.next()? {
                    "set" => Some(Instruction::Set {
                        reg: parts.next()?.to_string(),
                        reg_or_val: parts.next()?.parse().unwrap(),
                    }),
                    "sub" => Some(Instruction::Sub {
                        reg: parts.next()?.to_string(),
                        reg_or_val: parts.next()?.parse().unwrap(),
                    }),
                    "mul" => Some(Instruction::Mul {
                        reg: parts.next()?.to_string(),
                        reg_or_val: parts.next()?.parse().unwrap(),
                    }),
                    "jnz" => Some(Instruction::Jnz {
                        check_reg_or_val: parts.next()?.to_string(),
                        jmp_reg_or_val: parts.next()?.parse().unwrap(),
                    }),
                    _ => None,
                }
            })
            .collect();
    }

    fn run(&mut self) {
        self.program_counter = 0;

        loop {
            if !self.step() {
                break;
            }
        }
    }

    fn step(&mut self) -> bool {
        match &self.program[self.program_counter] {
            Instruction::Set { reg, reg_or_val } => {
                let value = self.get_reg_val_or_val(reg_or_val);

                if let Some(val) = self.registers.get_mut(reg) {
                    *val = value;
                } else {
                    self.registers.insert(reg.clone(), value);
                }

                self.advance_program_counter(1)
            }
            Instruction::Sub { reg, reg_or_val } => {
                let value = self.get_reg_val_or_val(reg_or_val);

                if let Some(val) = self.registers.get_mut(reg) {
                    *val -= value;
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

                self.mul_count += 1;

                self.advance_program_counter(1)
            }
            Instruction::Jnz {
                check_reg_or_val,
                jmp_reg_or_val,
            } => {
                let check_value = self.get_reg_val_or_val(check_reg_or_val);

                if check_value != 0 {
                    let jmp_value = self.get_reg_val_or_val(jmp_reg_or_val);

                    self.advance_program_counter(jmp_value as isize)
                } else {
                    self.advance_program_counter(1)
                }
            }
        }
    }

    fn advance_program_counter(&mut self, step: isize) -> bool {
        if let Some(new_val) = self.program_counter.checked_add_signed(step) {
            self.program_counter = new_val;
            self.program_counter < self.program.len()
        } else {
            false
        }
    }

    fn get_reg_val_or_val(&self, reg_or_val: &str) -> i64 {
        reg_or_val
            .parse()
            .unwrap_or_else(|_| *self.registers.get(reg_or_val).unwrap_or(&0))
    }
}

pub fn part1() {
    let program_text = std::fs::read_to_string("data/day23.txt").unwrap();

    let mut program = Program::new();
    program.load(&program_text);

    program.run();

    println!("{}", program.mul_count);
}

pub fn part2() {
    let mut h = 0;

    let b = 81 * 100 + 100000;
    let c = b + 17000;

    for b in (b..=c).step_by(17) {
        let mut f = 1;

        'outer: for d in 2..=b {
            if b % d != 0 {
                continue;
            }

            for e in 2..=b {
                if d * e == b {
                    f = 0;
                    break 'outer;
                }
            }
        }

        if f == 0 {
            h += 1;
        }
    }

    println!("{}", h);
}
