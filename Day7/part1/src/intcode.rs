use std::collections::VecDeque;

enum Modes {
    Position = 0,
    Immediate = 1,
}

pub struct IntcodeComputer {
    inputs: VecDeque<i32>,
    pub output: i32,
    opcodes: Vec<i32>
}

impl IntcodeComputer{
    pub fn new(inputs: VecDeque<i32>, opcodes: &Vec<i32>) -> IntcodeComputer {
        IntcodeComputer {
            inputs,
            output: 0,
            opcodes: opcodes.clone()
        }
    }

    pub fn run(&mut self) -> i32 {
        let mut pc = 0;
        loop {
            let mut opcode: usize = self.opcodes[pc] as usize;
            let mut instruction = opcode % 100;
            opcode /= 100;
            let mut param1_mode = opcode % 10;
            opcode /= 10;
            let mut param2_mode = opcode % 10;
            opcode /= 10;
            let mut param3_mode = opcode % 10;

            let mut param1;
            let mut param2;
            let mut destination;
            match instruction {
                1 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[pc + 3]; 

                    self.opcodes[destination as usize] = param1 + param2;
                    pc += 4;
                },
                2 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[pc + 3];

                    self.opcodes[destination as usize] = param1 * param2;
                    pc += 4;
                },
                3 => {
                    destination = self.opcodes[pc + 1];
                    self.opcodes[destination as usize] = self.inputs.pop_front().unwrap();
                    pc += 2;
                },
                4 => {
                    destination = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        destination = self.opcodes[destination as usize];
                    }

                    println!("{}", destination);
                    self.output = destination;
                    pc += 2;
                },
                5 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    if param1 != 0 {
                        pc = param2 as usize;
                    } else {
                        pc += 3;
                    }
                },
                6 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    if param1 == 0 {
                        pc = param2 as usize;
                    } else {
                        pc += 3;
                    }
                },
                7 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[pc + 3];

                    if param1 < param2 {
                        self.opcodes[destination as usize] = 1;
                    } else {
                        self.opcodes[destination as usize] = 0;
                    }
                    pc += 4;
                },
                8 => {
                    param1 = self.opcodes[pc + 1];
                    if param1_mode == Modes::Position as usize {
                        param1 = self.opcodes[param1 as usize];
                    }

                    param2 = self.opcodes[pc + 2];
                    if param2_mode == Modes::Position as usize {
                        param2 = self.opcodes[param2 as usize];
                    }

                    destination = self.opcodes[pc + 3];

                    if param1 == param2 {
                        self.opcodes[destination as usize] = 1;
                    } else {
                        self.opcodes[destination as usize] = 0;
                    }
                    pc += 4;
                },
                99 => break,
                _ => panic!("Invalid opcode: ({}) {}", pc, self.opcodes[pc])
            }
        }

        self.output
    }
}
