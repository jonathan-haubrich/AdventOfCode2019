use std::env;
use std::fs;
use std::process;

struct Config {
    input: String
}

impl Config {
    fn new(args : &Vec<String>) -> Result<Config, String> {
        if args.len() < 2 {
            return Err(format!("Usage: {} <input>", args[0]));
        }

        let input = args[1].clone();

        Ok(Config { input })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Config.input: {}", config.input);

    let contents = fs::read_to_string(config.input).unwrap_or_else(|err| {
        println!("read_to_string failed: {}", err);
        process::exit(1);
    });

    let numbers: Vec<&str> = contents.trim()
        .split('\n')
        .collect();

    let sum: u32 = numbers.into_iter().map(|x| { calculate_fuel(x.parse::<u32>().unwrap()) }).sum();

    println!("Sum: {}", sum);

    println!("Test: {}", calculate_fuel(1969));
}

fn calculate_fuel(mass: u32) -> u32 {
    if mass / 3 < 2 {
        return 0;
    }

    return (mass / 3 - 2) + calculate_fuel(mass / 3 - 2);
}
