
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let mem: Vec<usize> = fs::read_to_string(&args[1]).unwrap()
        .trim()
        .split(',')
        .map(|s| { s.parse().unwrap() })
        .collect();

    let target = 19690720;
    for i in 0..100 {
        for j in 0..100 {
            let mut opcodes = mem.to_vec();
            let mut pc = 0;
            opcodes[1] = i;
            opcodes[2] = j;
                loop {
                    let op1: usize = opcodes[pc + 1];
                    let op2: usize = opcodes[pc + 2];
                    let index: usize = opcodes[pc + 3];
                    match opcodes[pc] {
                        1 => {
                            opcodes[index] = opcodes[op1] + opcodes[op2]
                        },
                        2 => {
                            opcodes[index] = opcodes[op1] * opcodes[op2]
                        },
                        99 => {
                            if opcodes[0] == target {
                                println!("{:?}", opcodes);
                            }
                            break;
                        },
                        _ => panic!{"Invalid opcode: ({}) {}", pc, opcodes[pc]}
                    }
                    pc += 4;
                }
        }
    }
}
