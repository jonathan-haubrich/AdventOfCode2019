use std::env;
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let file = fs::File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
    let lines: Vec<String> = file.lines().map(|l| l.unwrap().trim().to_string()).collect();

    let input = &lines[0];
    let input = input.split("-").collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();

    let min = input[0];
    let max = input[1];

    let mut matched_criteria = 0;

    for i in min..=max {
        let mut remainder = i;
        let mut neighbors_matched = 0;
        let mut matching_neighbors = false;
        let mut ascending = true;
        let mut current_digit;
        let mut next_digit;
        while remainder > 0 {
            current_digit = (remainder / 10) % 10;
            next_digit = remainder % 10;
            if current_digit > next_digit {
                ascending = false;
            }

            if current_digit == next_digit {
                neighbors_matched += 1;
            } else {
                if neighbors_matched == 1 {
                    matching_neighbors = true;
                }
                neighbors_matched = 0;
            }
            
            remainder /= 10;
        }
        if matching_neighbors && ascending {
            matched_criteria += 1;
        }
    }

    println!("{}", matched_criteria);
}
