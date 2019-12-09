
use std::env;
use std::fs;
use std::process;

enum Modes {
    Position = 0,
    Immediate = 1,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input> <input(s)>", args[0]);
        process::exit(1);
    }

    let input = args[2].parse().unwrap();
    println!("input: {}", input);

    let mem: Vec<i32> = fs::read_to_string(&args[1]).unwrap()
        .trim()
        .split(',')
        .map(|s| { s.parse().unwrap_or_else(|err| {
            eprintln!("Failed to unwrap: {}", s);
            process::exit(1);
        })})
        .collect();

    let mut opcodes = mem.to_vec();
    let mut pc = 0;
    loop {
        let mut opcode: usize = opcodes[pc] as usize;
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
                param1 = opcodes[pc + 1];
                if param1_mode == Modes::Position as usize {
                    param1 = opcodes[param1 as usize];
                }

                param2 = opcodes[pc + 2];
                if param2_mode == Modes::Position as usize {
                    param2 = opcodes[param2 as usize];
                }

                destination = opcodes[pc + 3]; 

                opcodes[destination as usize] = param1 + param2;
                pc += 4;
            },
            2 => {
                param1 = opcodes[pc + 1];
                if param1_mode == Modes::Position as usize {
                    param1 = opcodes[param1 as usize];
                }

                param2 = opcodes[pc + 2];
                if param2_mode == Modes::Position as usize {
                    param2 = opcodes[param2 as usize];
                }

                destination = opcodes[pc + 3];

                opcodes[destination as usize] = param1 * param2;
                pc += 4;
            },
            3 => {
                destination = opcodes[pc + 1];
                opcodes[destination as usize] = input;
                pc += 2;
            },
            4 => {
                destination = opcodes[pc + 1];
                println!("{}", opcodes[destination as usize]);
                pc += 2;
            },
            99 => break,
            _ => panic!("Invalid opcode: ({}) {}", pc, opcodes[pc])
        }
    }
}
