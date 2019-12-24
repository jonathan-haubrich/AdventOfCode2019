use std::cell::{RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
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

    let com = planets.get("SAN").unwrap().borrow();

    let mut next = VecDeque::new();
    let mut path: VecDeque<String> = VecDeque::new();
    let mut paths: Vec<VecDeque<String>> = Vec::new();
    let mut visited = HashSet::new();
    const TARGET: &str = "YOU";

    next.push_back(com.name.clone());
    path.push_front(com.name.clone());
    visited.insert(com.name.clone());


    loop {
        match next.pop_front() {
            Some(name) => {
                visited.insert(name.clone());
                path.push_front(name.clone());
                if name == TARGET {
                    paths.push(path.clone());
                    path.pop_front();
                    continue;
                }

                let mut pushed = 0;
                match planets.get(&name) {
                    Some(planet) => {
                        let p = planet.borrow();
                        match &p.orbits {
                            Some(n) => {
                                if !visited.contains(n) {
                                    next.push_back(n.clone());
                                    pushed += 1;
                                }
                            },
                            None => {}
                        };
                        for orbiter in &p.orbited_by {
                            if !visited.contains(orbiter) {
                                next.push_back(orbiter.clone());
                                pushed += 1;
                            }
                        }
                        if pushed == 0 {
                            path.pop_front();
                        }
                    },
                    None => break
                }
            },
            None => break
        };
    }

    let mut shortest = !0;

    for i in 0..paths.len() {
        if paths[i].len() < shortest {
            shortest = i;
        }
    }

    println!("{:#?}", paths[shortest]);
    println!("Hops: {}", paths[shortest].len() - 2);
}
