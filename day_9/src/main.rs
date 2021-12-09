use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone)]
struct DepthMap {
    map: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, PartialEq)]
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

    fn get_neighbors_and_point(&self, input: &Point) -> Vec<(Point, i32)> {
        let mut neighbors = Vec::new();

        let nrow = self.nrows();
        let ncol = self.ncols();

        if input.x > 0 {
            let temp_point = Point {
                x: input.x - 1,
                y: input.y,
            };
            neighbors.push((temp_point.clone(), self.val_at(&temp_point)));
        }
        if input.x < ncol - 1 {
            let temp_point = Point {
                x: input.x + 1,
                y: input.y,
            };
            neighbors.push((temp_point.clone(), self.val_at(&temp_point)));
        }
        if input.y > 0 {
            let temp_point = Point {
                x: input.x,
                y: input.y - 1,
            };
            neighbors.push((temp_point.clone(), self.val_at(&temp_point)));
        }
        if input.y < nrow - 1 {
            let temp_point = Point {
                x: input.x,
                y: input.y + 1,
            };
            neighbors.push((temp_point.clone(), self.val_at(&temp_point)));
        }

        neighbors
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

    let mut basin_location: Vec<Point> = Vec::new();
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
                basin_location.push(Point { x, y });
            }
        }
    }
    println!("count: {}", risk_level);

    //Part 2
    println!("Basins: {:?}", basin_location);

    let mut len_list = Vec::new();
    for basin in &basin_location {
        let mut basin_list: Vec<Point> = Vec::new();
        let mut check_list: Vec<Point> = Vec::new();

        check_list.push(basin.clone());
        basin_list.push(basin.clone());

        loop {
            if check_list.is_empty() {
                break;
            }

            let neighbor_list = input.get_neighbors_and_point(&check_list[0]);

            for item in &neighbor_list {
                if item.1 != 9 && !basin_list.contains(&item.0) {
                    basin_list.push(item.0.clone());
                    check_list.push(item.0.clone());
                }
            }
            check_list.remove(0);
        }
        len_list.push(basin_list.len());
        // println!("Basin: {:?}", basin_list);
        // println!("Basin Size: {:?}", basin_list.len());
    }

    len_list.sort_by(|a, b| b.cmp(a));
    println!("{:?}", &len_list[0..3]);
}
