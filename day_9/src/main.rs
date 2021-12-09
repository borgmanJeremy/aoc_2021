use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct DepthMap {
    map: Vec<Vec<i32>>,
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl DepthMap {
    fn new() -> Self {
        Self { map: Vec::new() }
    }

    fn print(&self) {
        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                print!("{}", self.val_at(&Point { x, y }));
            }
            println!();
        }
    }
    fn val_at(&self, input: &Point) -> i32 {
        self.map[input.y][input.x]
    }

    fn ncols(&self) -> usize {
        self.map[0].len()
    }

    fn nrows(&self) -> usize {
        self.map.len()
    }

    fn get_neighbors(&self, input: &Point) -> Vec<i32> {
        let mut neighbors = Vec::new();

        let nrow = self.nrows();
        let ncol = self.ncols();

        if input.x > 0 {
            neighbors.push(self.val_at(&Point {
                x: input.x - 1,
                y: input.y,
            }));
        }
        if input.x < ncol - 1 {
            neighbors.push(self.val_at(&Point {
                x: input.x + 1,
                y: input.y,
            }));
        }
        if input.y > 0 {
            neighbors.push(self.val_at(&Point {
                x: input.x,
                y: input.y - 1,
            }));
        }
        if input.y < nrow - 1 {
            neighbors.push(self.val_at(&Point {
                x: input.x,
                y: input.y + 1,
            }));
        }

        neighbors
    }
}

fn read_from_file(path: &str) -> DepthMap {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut map = DepthMap::new();

    map.map = reader
        .lines()
        .map(|line| {
            if let Ok(text) = line {
                text.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            } else {
                panic!("Failed to parse");
            }
        })
        .collect();
    map
}

fn main() {
    let input = read_from_file("input/input.txt");
    input.print();

    // Part 1
    let mut risk_level = 0;
    for x in 0..input.ncols() {
        for y in 0..input.nrows() {
            let neighbors = input.get_neighbors(&Point { x, y });
            let mut highest = true;
            for n in &neighbors {
                if input.val_at(&Point { x, y }) >= *n {
                    highest = false;
                }
            }
            if highest {
                risk_level += 1 + input.val_at(&Point { x, y });
            }
        }
    }
    println!("count: {}", risk_level);
}
