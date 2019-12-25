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
    println!("input: {}", input);

    let mem: Vec<i32> = fs::read_to_string(&args[1]).unwrap()
        .trim()
        .split(',')
        .map(|s| { s.trim().parse().unwrap_or_else(|err| {
            eprintln!("Failed to unwrap: {}", s);
            process::exit(1);
        })})
        .collect();

    let phase_settings: &[&i32] = &[&0, &1, &2, &3, &4];
    let phase_settings = [phase_settings];

    let mut permutator = Permutator::new(&phase_settings[..]);
    let mut result = 0;
    let mut outputs = Vec::new();

    loop {
        result = 0;
        match permutator.next() {
            Some(p) => {
                let mut unique: Vec<i32> = p.iter().map(|x| **x).collect();
                let mut sorted_unique = unique.clone();
                sorted_unique.sort();
                sorted_unique.dedup();
                if sorted_unique.len() == 5 {
                    for i in unique.into_iter() {
                        let mut inputs = VecDeque::new();
                        inputs.push_back(i);
                        inputs.push_back(result);
                        let mut ic = IntcodeComputer::new(inputs, &mem);
                        ic.run();
                        result = ic.output;
                    }
                    outputs.push(result);
                }
            },
            None => break
        };
    }

    println!("Outputs: {:#?}\nMin: {}", outputs, outputs.iter().max().unwrap());

}
