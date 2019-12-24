use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};
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

    let mut planets: HashMap<String, RefCell<Planet>> = HashMap::new();

    let names: Vec<(String, String)> = file.lines().map(|s| {
        let names: Vec<String> = s.unwrap()
            .trim()
            .split(")")
            .map(|s| s.to_string())
            .collect();
        (names[0].clone(), names[1].clone())})
        .collect();

    for (name1, name2) in &names {
        planets.insert(name1.to_owned(), RefCell::new(Planet::new(name1, None)));
        planets.insert(name2.to_owned(), RefCell::new(Planet::new(name2, None)));
    }

    for (name1, name2) in &names {
        let p1 = planets.get(name1).unwrap();
        let p2 = planets.get(name2).unwrap();

        Planet::add_orbiter(&mut *p1.borrow_mut(), p2.borrow().name.clone());
        Planet::orbit(&mut *p2.borrow_mut(), p1.borrow().name.clone());
    }

    let mut path: HashSet<String> = HashSet::new();
    let mut paths: Vec<HashSet<String>> = Vec::new();
    let mut starts = Vec::new();

    match planets.get("YOU") {
        Some(p) => {
            let planet = p.borrow();
            match &planet.orbits {
                Some(n) => {
                    starts.push(n.clone());
                },
                None => {}
            };
        },
        None => {}
    }

    starts.push(String::from("SAN"));

    for start in starts {
        path.clear();
        let mut current: String = start.to_string();
        loop {
            match planets.get(&current) {
                Some(p) => {
                    let planet = p.borrow();
                    match &planet.orbits {
                        Some(n) => {
                            path.insert(n.clone());
                            current = n.clone();
                        },
                        None => {
                            paths.push(path.clone());
                            break;
                        }
                    };
                },
                None => break
            }
        }
    }

    let path_to_santa = &paths[1].symmetric_difference(&paths[0]).collect::<Vec<&String>>();

    println!("{:#?}", path_to_santa);
    // +1 because the path will contain a planet that is shared by both paths
    println!("Hops: {}", path_to_santa.len() + 1);
}
