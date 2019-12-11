use std::boxed::{Box};
use std::cell::{RefCell};
use std::collections::{HashMap};
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

    let mut planets: HashMap<String, RefCell<Box<Planet>>> = HashMap::new();

    let names: Vec<(String, String)> = file.lines().map(|s| {
        let names: Vec<String> = s.unwrap()
            .trim()
            .split(")")
            .map(|s| s.to_string())
            .collect();
        (names[0].clone(), names[1].clone())})
        .collect();

    for (name1, name2) in names {

        let p1 = match planets.get(&name1) {
            Some(p) => p.clone(),
            None => RefCell::new(Box::new(Planet::new(&name1, None)))
        };

        let p2 = match planets.get(&name2) {
            Some(p) => p.clone(),
            None => RefCell::new(Box::new(Planet::new(&name2, None)))
        };

        Planet::add_orbiter(p1.borrow_mut(), p2.clone());
        Planet::orbit(p2.borrow_mut(), p1.clone());

        planets.insert(name1.clone(), p1.clone());
        planets.insert(name2.clone(), p2.clone());
    }

//    for line in lines {


    println!("{:#?}", planets);

}
