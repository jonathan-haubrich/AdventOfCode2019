use std::cmp;
use std::env;
use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::process;
use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
struct Coord {
    x: i32,
    y: i32
}

struct Grid {
    length: i32,
    fill: char,
    rows: Vec<Vec<char>>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", args[0]);
        process::exit(1);
    }

    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);
//    println!("{:?}", lines);
    //let lines = file.lines();
    let lines: Vec<String> = file
        .lines()
        .map(|s| -> String { 
            String::from(s.unwrap().trim())
        })
        .collect();


    let mut grid = Grid::new(' ');

    for line in lines {
        let mut position = Coord::new(0, 0);
        let moves: Vec<String> = line.trim()
            .split(",")
            .map(|s| String::from(s))
            .collect();
        grid.walk(&mut position, &moves);
    }

    for row in grid.rows.iter().rev() {
        println!("{:?}", row);
    }
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y}
    }
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Coord) -> Self::Output {
        Self { 
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Grid {
    fn new(fill: char) -> Grid {
        Grid { length: 0, fill: fill, rows: vec![vec![fill; 0]; 0] }
    }

    fn expand(&mut self, increase: i32) {
        let new_length = self.length + increase.abs();

        if increase < 0 {
            for row in &mut self.rows {
                for _ in 0..increase.abs() {
                    row.insert(0,self.fill);
                }
            }
            for _ in 0..increase.abs() {
                self.rows.insert(0, vec![self.fill; new_length as usize]);
            }
        } else {
            for row in &mut self.rows {
                row.extend(vec![self.fill; increase as usize]);
            }
            for _ in 0..increase.abs() {
                self.rows.push(vec![self.fill; new_length as usize]);
            }
        }

        self.length = new_length;
    }

    fn update(&mut self, position: &Coord, marker: char) {
        let y: usize = position.y as usize;
        let x: usize = position.x as usize;
        let row: &mut Vec<char> = &mut self.rows[y];
        row[x] = marker;
    }

    fn walk(&mut self, position: &mut Coord, path: &Vec<String>) {
        let mut end_y: i32 = 0;
        let mut end_x: i32 = 0;
        let mut grid_limit: i32;
        for instruction in path.iter() {
            let direction = instruction.chars().nth(0).unwrap();
            let distance = &instruction[1..];
            let distance = distance.parse().unwrap_or_else(|err| {
                eprintln!("Failed to parse number ({}): {}", distance, err);
                process::exit(1);
            });

            let marker: char;
            let mut delta = Coord::new(0, 0);

            println!("instruction: {:?}", instruction);
            match direction {
                'U' => {
                    marker = '|';
                    delta.y = 1;
                    delta.x = 0;
                    end_y += distance;
                },
                'L' => {
                    marker = '-';
                    delta.x = -1;
                    delta.y = 0;
                    end_x -= distance;
                },
                'D' => {
                    marker = '|';
                    delta.y = -1;
                    delta.x = 0;
                    end_y -= distance;
                },
                'R' => {
                    marker = '-';
                    delta.x = 1;
                    delta.y = 0;
                    end_x += distance;
                },
                _ => panic!("Invalid direction: {}", direction)
            }

            grid_limit = cmp::max(end_x, end_y);
            println!("grid_limit: {}, distance: {}", grid_limit, distance);
            if grid_limit > self.length {
                self.expand((grid_limit - self.length) + 1);
            }

            grid_limit = cmp::min(end_x, end_y);
            if grid_limit < 0 {
                match marker {
                    '|' => position.y += grid_limit.abs(),
                    '-' => position.x += grid_limit.abs(),
                    _ => panic!("Invalid marker for some reason...")
                }
            }
            for _ in 0..distance {
                *position += delta;
                println!("{} {}", position.x, position.y);
                self.update(&position, marker);
            }
            println!("Finished distance loop");
        }
    }
}
