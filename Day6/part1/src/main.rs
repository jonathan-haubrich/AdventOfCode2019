use std::env;
use std::fs;
use std::io::{BufReader, prelude::*};
use std::process;

use planet::Planet;
pub mod planet;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    println!("{}", args[1]);

    let file = fs::File::open(&args[1]).unwrap();
    let file = BufReader::new(file);

    let lines: Vec<String> = file.lines().map(|s| s.unwrap()).collect();

    for line in lines {
        println!("{}", line);
    }

    let name = String::from("MC168");
    let planet = Planet::new(&name, None);

}
