use std::env;
use std::cmp;
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::ops::{Add, AddAssign};
use std::process;

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: i32,
    y: i32
}

#[derive(Copy, Clone, Debug)]
struct Line {
    p1: Coord,
    p2: Coord
}

#[derive(Clone, Debug)]
struct Wire {
    path: Vec<Line>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap_or_else(|err| {
        eprintln!("Failed to open file {}: {}", args[1], err);
        process::exit(1);
    });
    let file = BufReader::new(file);
//    println!("{:?}", lines);
    //let lines = file.lines();
    let wires: Vec<String> = file
        .lines()
        .map(|s| -> String { 
            String::from(s.unwrap().trim())
        })
        .collect();
        

    let mut all_wires: Vec<Wire> = Vec::new();
    for wire in wires {
        let mut p1 = Coord::new(0, 0);
        let mut p2 = Coord::new(0, 0);
        let mut current_wire = Wire::new();
        let mut current_path = Line::new(p1, p2);
        let lines: Vec<String> = wire.trim()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        for line in lines {
            p1 = p2;
            let direction = line.chars().nth(0).unwrap();
            let distance = &line[1..];
            let distance: i32 = distance.parse().unwrap();
            translate_direction(direction, distance, &mut p2);
            current_path.update(&p1, &p2);
            current_wire.add_line(current_path);
        }
        all_wires.push(current_wire);
    }

    /*
    for wire in all_wires {
        println!("========== Wire ==========");
        for path in wire.path.iter() {
            println!("{:?}", path);
        }
    }*/

    let intersections = find_intersections(&all_wires[0], &all_wires[1]);

    let mut min_distance = std::i32::MAX;
    let mut distance;
    for intersection in intersections {
        distance = intersection.x.abs() + intersection.y.abs();
        if distance != 0 && distance < min_distance {
            min_distance = distance;
        }
    }

    println!("{}", min_distance);
    
}

fn find_intersections(wire1: &Wire, wire2: &Wire) -> Vec<Coord> {
    let mut intersections: Vec<Coord> = Vec::new();
    let mut intersection: Coord = Coord::new(0, 0);
                   
    for lines1 in &wire1.path {
        for lines2 in &wire2.path {
            if get_intersection(&lines1, &lines2, &mut intersection) {
//                println!("{:?} {:?} {:?}", lines1, lines2, intersection);
                intersections.push(intersection);
            }
        }
    }
    intersections
}

fn get_intersection(line1: &Line, line2: &Line, point: &mut Coord) -> bool {
    // Find either the horizontal or vertical line
    // We find vertical first
    let vertical: &Line;
    let horizontal: &Line;

    if line1.p1.y == line1.p2.y {
        horizontal = &line1;
        vertical = &line2;
    } else if line2.p1.y == line2.p2.y {
        horizontal = &line2;
        vertical = &line1;
    } else {
        // both lines are vertical
        return false;
    }

    // Check to make sure lines are not parallel
    if horizontal.p1.y == vertical.p1.y && horizontal.p2.y == vertical.p2.y {
        return false;
    }

    let vertical_max_y = cmp::max(vertical.p1.y, vertical.p2.y);
    let vertical_min_y = cmp::min(vertical.p1.y, vertical.p2.y);

    if horizontal.p1.y > vertical_max_y || horizontal.p1.y < vertical_min_y {
        return false;
    }

    point.x = vertical.p1.x;
    point.y = horizontal.p1.y;
    true
}

fn translate_direction(direction: char, distance: i32, position: &mut Coord) {
    match direction {
        'U' => position.y += distance,
        'R' => position.x += distance,
        'D' => position.y -= distance,
        'L' => position.x -= distance,
        _ => panic!("Invalid direction")
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y}
    }

    fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, other: Coord) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Line {
    fn new(p1: Coord, p2: Coord) -> Line {
        Line {
            p1: p1,
            p2: p2
        }
    }

    fn update(&mut self, p1: &Coord, p2: &Coord) {
        self.p1 = *p1;
        self.p2 = *p2;
    }

    fn is_vertical(self) -> bool {
        return self.p1.x == self.p2.x;
    }

    fn is_horizontal(self) -> bool {
        return self.p1.y == self.p2.y;
    }

    fn intersects(self, other: &Line) -> bool {
        let lines = [&self, other];
        if lines.iter().all(|&l| l.is_vertical())
            || lines.iter().all(|&l| l.is_horizontal()) {
            return false;
        }

        true
    }
}

impl Wire {
    fn new() -> Wire {
        Wire {
            path: Vec::new()
        }
    }

    fn add_line(&mut self, line: Line) {
        self.path.push(line);
    }
}
