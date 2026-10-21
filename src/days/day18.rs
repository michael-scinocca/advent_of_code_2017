use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex},
};

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
    pid: i64,
    registers: HashMap<String, i64>,
    program: Vec<Instruction>,
    program_counter: usize,
    last_frequency: i64,
    send_count: i64,
    is_receiving: bool,
    incoming: Arc<Mutex<VecDeque<i64>>>,
    outgoing: Option<Arc<Mutex<VecDeque<i64>>>>,
}

impl Program {
    fn new(pid: i64) -> Self {
        let mut program = Program {
            pid,
            registers: HashMap::new(),
            program: Vec::new(),
            program_counter: 0,
            last_frequency: 0,
            send_count: 0,
            is_receiving: false,
            incoming: Arc::new(Mutex::new(VecDeque::new())),
            outgoing: None,
        };

        if pid >= 0 {
            program.registers.insert("p".to_string(), pid);
        }

        program
    }

    fn load(&mut self, program: &str, outgoing: Option<Arc<Mutex<VecDeque<i64>>>>) {
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

        if outgoing.is_none() {
            self.outgoing = Some(self.incoming.clone())
        } else {
            self.outgoing = outgoing;
        }
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
        self.is_receiving = false;

        match &self.program[self.program_counter] {
            Instruction::Snd { reg } => {
                if let Some(val) = self.registers.get(reg)
                    && let Some(outgoing) = &mut self.outgoing
                {
                    let mut outgoing = outgoing.lock().unwrap();
                    outgoing.push_back(*val);
                    self.send_count += 1;
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

                if self.pid < 0 {
                    if value != 0 {
                        let mut incoming = self.incoming.lock().unwrap();
                        self.last_frequency = incoming.pop_back().unwrap_or(0);
                        false
                    } else {
                        self.advance_program_counter(1)
                    }
                } else {
                    let mut incoming = self.incoming.lock().unwrap();

                    if incoming.is_empty() {
                        self.is_receiving = true;
                        true
                    } else {
                        let received = incoming.pop_front().unwrap();

                        drop(incoming);

                        if let Some(val) = self.registers.get_mut(reg_or_val) {
                            *val = received;
                        } else {
                            self.registers.insert(reg_or_val.clone(), received);
                        }

                        self.advance_program_counter(1)
                    }
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

    let mut program = Program::new(-1);
    program.load(&program_text, None);

    program.run();

    println!("{}", program.last_frequency);
}

pub fn part2() {
    let program_text = std::fs::read_to_string("data/day18.txt").unwrap();

    let mut program0 = Program::new(0);
    let mut program1 = Program::new(1);

    program0.load(&program_text, Some(program1.incoming.clone()));
    program1.load(&program_text, Some(program0.incoming.clone()));

    loop {
        let mut continue_processing = false;

        continue_processing |= program0.step();
        continue_processing |= program1.step();

        if program0.is_receiving && program1.is_receiving {
            continue_processing = false;
        }

        if !continue_processing {
            break;
        }
    }

    println!("{}", program1.send_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut program = Program::new(-1);
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
            None,
        );

        program.run();

        assert_eq!(4, program.last_frequency);
    }
}
