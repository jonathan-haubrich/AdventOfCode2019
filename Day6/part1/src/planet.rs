use std::cell::{RefCell, RefMut};

#[derive(Debug, Clone)]
pub struct Planet {
    pub name: String,
    orbits: Option<RefCell<Box<Planet>>>,
    orbited_by: Vec<RefCell<Box<Planet>>>
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
    pub fn new(name: &str, orbits: Option<RefCell<Box<Planet>>>) -> Planet {
        Planet {
            name: name.to_string(),
            orbits,
            orbited_by: Vec::new()
        }
    }

    pub fn add_orbiter(mut planet: RefMut<Box<Planet>>, orbiter: RefCell<Box<Planet>>) {
        planet.orbited_by.push(orbiter);
    }

    pub fn orbit(mut planet: RefMut<Box<Planet>>, center: RefCell<Box<Planet>>) {
        planet.orbits = Some(center);
    }
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
