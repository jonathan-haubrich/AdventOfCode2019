use std::env;
use std::fs;
use std::io;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input>",
            args[0]);
        process::exit(1);
    }

    for arg in args {
        println!("{}", arg);
    }
}
