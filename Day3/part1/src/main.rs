use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
//    println!("{:?}", lines);
    //let lines = file.lines();
    let lines: Vec<String> = file
        .lines()
        .map(|s| -> String { 
            String::from(s.unwrap().trim())
        })
        .collect();

    let mut moves: Vec<String> = Vec::new();
    for line in lines {
        for instruction in line.split(',') {
            moves.push(String::from(instruction));
        }
    }

    let mut max = 0;
    let mut current;
    for instruction in moves.iter() {
        let distance = &instruction[1..];
        current = distance.parse().unwrap_or_else(|err| {
            eprintln!("Failed to parse number {}: {}", distance, err);
            process::exit(1);
        });
        if current > max {
            max = current;
        }
    }

    println!("{:?}", max);
}
