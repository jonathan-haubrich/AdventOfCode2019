use std::collections::VecDeque;

enum Modes {
    Position = 0,
}

#[derive(Debug)]
pub struct IntcodeComputer {
    pub id: i32,
    pub inputs: VecDeque<i32>,
    pub outputs: VecDeque<i32>,
    opcodes: Vec<i32>,
    pub halted: bool,
    pub pc: usize,
}

impl IntcodeComputer{
    pub fn new(inputs: VecDeque<i32>, opcodes: &Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            id: *inputs.front().unwrap(),
            inputs,
            outputs: VecDeque::new(),
            opcodes: opcodes.clone(),
            halted: false,
            pc: 0
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut opcode: usize = self.opcodes[self.pc] as usize;
            let instruction = opcode % 100;
            opcode /= 100;
            let param1_mode = opcode % 10;
            opcode /= 10;
            let param2_mode = opcode % 10;

            let mut param1;
            let mut param2;
            let mut destination;
            match instruction {
                1 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[self.pc + 3]; 

                    self.opcodes[destination as usize] = param1 + param2;
                    self.pc += 4;
                },
                2 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[self.pc + 3];

                    self.opcodes[destination as usize] = param1 * param2;
                    self.pc += 4;
                },
                3 => {
                    if self.inputs.len() == 0 {
                        break;
                    }
                    destination = self.opcodes[self.pc + 1];
                    self.opcodes[destination as usize] = self.inputs.pop_front().unwrap();
                    self.pc += 2;
                },
                4 => {
                    destination = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        destination = self.opcodes[destination as usize];
                    }

                    //println!("{}", destination);
                    self.outputs.push_back(destination);
                    self.pc += 2;
                },
                5 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    if param1 != 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                6 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    if param1 == 0 {
                        self.pc = param2 as usize;
                    } else {
                        self.pc += 3;
                    }
                },
                7 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[self.pc + 3];

                    if param1 < param2 {
                        self.opcodes[destination as usize] = 1;
                    } else {
                        self.opcodes[destination as usize] = 0;
                    }
                    self.pc += 4;
                },
                8 => {
                    param1 = self.opcodes[self.pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[self.pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[self.pc + 3];

                    if param1 == param2 {
                        self.opcodes[destination as usize] = 1;
                    } else {
                        self.opcodes[destination as usize] = 0;
                    }
                    self.pc += 4;
                },
                99 => {
                    self.halted = true;
                    break;
                },
                _ => panic!("Invalid opcode: ({}) {}", self.pc, self.opcodes[self.pc])
            }
        }
    }
}
