extern crate permutate;
use permutate::Permutator;

use std::collections::VecDeque;
use std::env;
use std::fs;
use std::process;

mod intcode;
use intcode::IntcodeComputer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input> <input(s)>", args[0]);
        process::exit(1);
    }

    let input: i32 = args[2].parse().unwrap();

    let mem: Vec<i32> = fs::read_to_string(&args[1]).unwrap()
        .trim()
        .split(',')
        .map(|s| { s.trim().parse().unwrap_or_else(|err| {
            eprintln!("{}: Failed to unwrap: {}", err, s);
            process::exit(1);
        })})
        .collect();

    let phase_settings: &[&i32] = &[&5, &6, &7, &8, &9];
    let phase_settings = [phase_settings];

    let mut permutator = Permutator::new(&phase_settings[..]);
    let mut outputs: VecDeque<i32> = VecDeque::new();
    let mut computers: VecDeque<IntcodeComputer>;
    let mut c1: IntcodeComputer;
    let mut c2: IntcodeComputer;

    loop {
        computers = VecDeque::new();
        match permutator.next() {
            Some(p) => {
                let unique: Vec<i32> = p.iter().map(|x| **x).collect();
                let mut sorted_unique = unique.clone();
                sorted_unique.sort();
                sorted_unique.dedup();
                if sorted_unique.len() == 5 {
                    for i in unique.iter() {
                        let mut inputs = VecDeque::new();
                        inputs.push_back(*i);
                        let ic = IntcodeComputer::new(inputs, &mem);
                        computers.push_back(ic);
                    }

                    computers.front_mut().unwrap().inputs.push_back(input);

                    while computers.iter().any(|c| { !c.halted }) {
                        c1 = computers.pop_front().unwrap();
                        c2 = computers.pop_front().unwrap();

                        c1.run();
                        c2.inputs.append(&mut c1.outputs);

                        computers.push_back(c1);
                        computers.push_front(c2);
                    }
                    outputs.append(&mut computers.front_mut().unwrap().inputs);
                }
            },
            None => break
        };
    }

    println!("Results: {:?}", outputs);
    println!("Max: {}", outputs.iter().max().unwrap());

    //println!("Outputs: {:#?}\nMin: {}", outputs);

}
