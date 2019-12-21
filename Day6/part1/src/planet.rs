use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    pub orbits: Option<String>,
    pub orbited_by: Vec<String>
}

/*
impl<'a> Planet<'a> {
    fn new(self, orbits: Option<&'a Planet>) -> Planet<'a> {
        Planet {
            orbits,
            orbited_by: Vec::<&'a Planet>::new(),
        }
    }
}
*/

impl Planet {
    pub fn new(name: &str, orbits: Option<String>) -> Planet {
        Planet {
            name: name.to_string(),
            orbits,
            orbited_by: Vec::new()
        }
    }

    pub fn add_orbiter(planet: &mut Planet, orbiter: String) {
        planet.orbited_by.push(orbiter);
    }

    pub fn orbit(planet: &mut Planet, center: String) {
        planet.orbits = Some(center);
    }

    pub fn indirect_orbits(center: &Planet, planets: &HashMap<String, RefCell<Planet>>) -> usize {
        let mut orbiters = VecDeque::new();
        let mut indirect = 0;

        for orbiter in &center.orbited_by {
            orbiters.push_back(orbiter.clone());
        }

        loop {
            match orbiters.pop_front() {
                Some(name) => {
                    match planets.get(&name) {
                        Some(p) => {
                            let planet = p.borrow();
                            for orbiter in &planet.orbited_by {
                                orbiters.push_back(orbiter.clone());
                                indirect += 1;
                            }
                        },
                        None => break
                    };
                },
                None => break
            }
        }

        indirect
    }
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
