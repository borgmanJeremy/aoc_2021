use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, PartialEq)]
struct EnergyCell {
    energy: i32,
    has_flashed: bool,
}
impl EnergyCell {
    fn new() -> Self {
        Self {
            energy: 0,
            has_flashed: false,
        }
    }
}

#[derive(Debug, Clone)]
struct EnergyMap {
    map: Vec<Vec<EnergyCell>>,
    total_flashes: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl EnergyMap {
    fn new() -> Self {
        Self {
            map: Vec::new(),
            total_flashes: 0,
        }
    }

    fn print(&self) {
        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                print!("{:?}", self.val_at(&Point { x, y }).energy);
            }
            println!();
        }
    }
    fn val_at(&self, input: &Point) -> EnergyCell {
        self.map[input.y][input.x].clone()
    }

    fn ncols(&self) -> usize {
        self.map[0].len()
    }

    fn nrows(&self) -> usize {
        self.map.len()
    }

    fn get_neighbors(&self, input: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        let nrow = self.nrows();
        let ncol = self.ncols();
        // Up Down Left Right
        if input.x > 0 {
            let temp_point = Point {
                x: input.x - 1,
                y: input.y,
            };
            neighbors.push(temp_point.clone());
        }
        if input.x < ncol - 1 {
            let temp_point = Point {
                x: input.x + 1,
                y: input.y,
            };
            neighbors.push(temp_point.clone());
        }
        if input.y > 0 {
            let temp_point = Point {
                x: input.x,
                y: input.y - 1,
            };
            neighbors.push(temp_point.clone());
        }
        if input.y < nrow - 1 {
            let temp_point = Point {
                x: input.x,
                y: input.y + 1,
            };
            neighbors.push(temp_point.clone());
        }

        // Diaganol
        if input.x > 0 && input.y > 0 {
            let temp_point = Point {
                x: input.x - 1,
                y: input.y - 1,
            };
            neighbors.push(temp_point.clone());
        }

        if input.x > 0 && input.y < nrow - 1 {
            let temp_point = Point {
                x: input.x - 1,
                y: input.y + 1,
            };
            neighbors.push(temp_point.clone());
        }

        if input.x < ncol - 1 && input.y > 0 {
            let temp_point = Point {
                x: input.x + 1,
                y: input.y - 1,
            };
            neighbors.push(temp_point.clone());
        }
        if input.x < ncol - 1 && input.y < nrow - 1 {
            let temp_point = Point {
                x: input.x + 1,
                y: input.y + 1,
            };
            neighbors.push(temp_point.clone());
        }
        neighbors
    }

    fn increment_neighbor(&mut self, input: &Point) {
        let neighbors = self.get_neighbors(&input);
        let mut new_flash_list = Vec::new();

        self.map[input.y][input.x].has_flashed = true;

        for cell in &neighbors {
            self.map[cell.y][cell.x].energy += 1;
            if self.map[cell.y][cell.x].energy > 9 && !self.map[cell.y][cell.x].has_flashed {
                new_flash_list.push(Point {
                    x: cell.x,
                    y: cell.y,
                });
            }
        }

        for cell in &new_flash_list {
            if !self.map[cell.y][cell.x].has_flashed {
                self.increment_neighbor(cell);
            }
        }
    }

    fn tick(&mut self) {
        let mut flash_list = Vec::new();

        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                self.map[y][x].energy += 1;
                self.map[y][x].has_flashed = false;
                if self.map[y][x].energy > 9 {
                    flash_list.push(Point { x, y });
                }
            }
        }

        for cell in &flash_list {
            if !self.map[cell.y][cell.x].has_flashed {
                self.increment_neighbor(cell);
            }
        }
        for y in 0..self.nrows() {
            for x in 0..self.ncols() {
                if self.map[y][x].energy > 9 {
                    self.map[y][x].energy = 0;

                    self.total_flashes += 1;
                }
            }
        }
    }
}

fn read_from_file(path: &str) -> EnergyMap {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut map = EnergyMap::new();

    map.map = reader
        .lines()
        .map(|line| {
            if let Ok(text) = line {
                text.chars()
                    .map(|c| EnergyCell {
                        energy: c.to_digit(10).unwrap() as i32,
                        has_flashed: false,
                    })
                    .collect::<Vec<EnergyCell>>()
            } else {
                panic!("Failed to parse");
            }
        })
        .collect();
    map
}

fn main() {
    let mut input = read_from_file("input/input.txt");
    println!("\nOriginal Map");
    input.print();

    let mut old_flash_count;
    for tick_count in 0..1000 {
        old_flash_count = input.total_flashes;
        input.tick();
        println!(
            "generation: {}, total flashes: {}",
            tick_count + 1,
            input.total_flashes
        );

        if input.total_flashes - old_flash_count == 100 {
            println!("found bring flash at generation: {}", tick_count + 1);
            break;
        }
    }
    println!("total flashes: {}", input.total_flashes);
}
