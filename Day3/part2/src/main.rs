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

    let mut intersections = find_intersections(&all_wires[0], &all_wires[1]);
    intersections.dedup();
    println!("{:#?}", intersections);

    println!("{:?}", &intersections[0]);

    let mut min_distance = std::i32::MAX;
    let mut distance;
    let mut start: Coord = Coord::new(0, 0);
    let mut end: &Coord;
    let mut current_wire: Wire;
    for i in 0..intersections.len() {
        end = &intersections[i];
        if start == *end {
            continue;
        }
        distance = 0;
        println!("intersection: {:?}", end);
        distance += get_path_distance(&all_wires[0], &start, &end);
        distance += get_path_distance(&all_wires[1], &start, &end);
        if distance < min_distance {
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
            if lines1.intersects(&lines2) {
//                println!("{:?} {:?} {:?}", lines1, lines2, intersection);
                if lines1.is_horizontal() {
                    intersection.y = lines1.p1.y;
                    intersection.x = lines2.p1.x;
                } else {
                    intersection.y = lines2.p1.y;
                    intersection.x = lines1.p1.x;
                }
                intersections.push(intersection);
            }
        }
    }
    intersections
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

//impl cmp::PartialEq for &Coord {
//    fn eq(&self, other: &Self) -> bool {
//        self.x == other.x
//            && self.y == other.y
//    }
//}

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

impl cmp::PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) 
            && (self.y == other.y)
    }
}

impl cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.p1.x == other.p1.x
            && self.p1.y == other.p1.y
            && self.p2.x == other.p2.x
            && self.p2.y == other.p2.y
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

        let vertical = if self.is_vertical() {
            &self
        } else {
            &other
        };

        let horizontal = if self.is_horizontal() {
            &self
        } else {
            &other
        };

        let vertical_min_y = cmp::min(vertical.p1.y, vertical.p2.y);
        let vertical_max_y = cmp::max(vertical.p1.y, vertical.p2.y);

        let horizontal_min_x = cmp::min(horizontal.p1.x, horizontal.p2.x);
        let horizontal_max_x = cmp::max(horizontal.p1.x, horizontal.p2.x);

        if vertical_min_y > horizontal.p1.y || vertical_max_y < horizontal.p1.y {
            return false;
        }

        if horizontal_min_x > vertical.p1.x || horizontal_max_x < vertical.p1.x {
            return false;
        }

        true
    }

    fn contains(self, point: &Coord) -> bool {
        let min_y = cmp::min(self.p1.y, self.p2.y);
        let max_y = cmp::max(self.p1.y, self.p2.y);

        let min_x = cmp::min(self.p1.x, self.p2.x);
        let max_x = cmp::max(self.p1.x, self.p2.x);

        if (point.y < min_y || point.y > max_y)
            || (point.x < min_x || point.x > max_x) {
            return false;
        }

        true        
    }

    fn get_distance(self) -> i32 {
        let length = if self.is_horizontal() {
            cmp::max(self.p1.x, self.p2.x) - cmp::min(self.p1.x, self.p2.x)
        } else {
            cmp::max(self.p1.y, self.p2.y) - cmp::min(self.p1.y, self.p2.y)
        };

        length.abs()
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

fn get_path_distance(wire: &Wire, start: &Coord, end: &Coord) -> i32 {
    let mut distance = 0;

    let start_index = wire.path.iter().position(|&l| {
        //println!("{:?} {:?} {:?}", l.p1, l.p2, start);
        l.contains(start)
    }).unwrap_or_else(|| {
        eprintln!("Getting start index failed: {:?}", start);
        return 0;
    });

    for line in &wire.path[start_index..] {
        if line.contains(end) {
            distance += get_manhattan_distance(&line.p1, end);
            break;
        }
        distance += line.get_distance();
    }

    distance
}

fn get_manhattan_distance(p1: &Coord, p2: &Coord) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}
