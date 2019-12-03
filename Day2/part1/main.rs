
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let mut opcodes: Vec<usize> = fs::read_to_string(&args[1]).unwrap()
        .trim()
        .split(',')
        .map(|s| { s.parse().unwrap() })
        .collect();

    let mut i = 0;
    loop {
        let op1: usize = opcodes[i + 1];
        let op2: usize = opcodes[i + 2];
        let index: usize = opcodes[i + 3];
        match opcodes[i] {
            1 => {
                opcodes[index] = opcodes[op1] + opcodes[op2]
            },
            2 => {
                opcodes[index] = opcodes[op1] * opcodes[op2]
            },
            99 => break,
            _ => panic!{"Invalid opcode: ({}) {}", i, opcodes[i]}
        }
        i += 4;
    }

    println!("{:?}", opcodes);
}
