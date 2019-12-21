use std::collections::{HashMap, VecDeque};
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

    let mut direct = 0;
    let mut indirect = 0;

    let com = planets.get("COM").unwrap().borrow();

    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(com.name.clone());

    loop {
        match queue.pop_front() {
            Some(name) => {
                match planets.get(&name) {
                    Some(planet) => {
                        let p = planet.borrow();
                        for orbiter in &p.orbited_by {
                            queue.push_back(orbiter.clone());
                            direct += 1;
                        }
                        indirect += Planet::indirect_orbits(&*p, &planets);
                    },
                    None => break
                }
            },
            None => break
        };
    }

    println!("Direct: {} Indirect: {} Total: {}", direct, indirect, direct + indirect);
}
