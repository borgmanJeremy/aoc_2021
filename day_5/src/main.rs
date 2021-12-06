use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

impl LineSegment {
    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn extend(&self) -> Option<Vec<Point>> {
        let mut full_line = Vec::new();
        if self.is_horizontal() {
            for x in cmp::min(self.p1.x, self.p2.x)..=cmp::max(self.p1.x, self.p2.x) {
                full_line.push(Point { x, y: self.p1.y });
            }
            return Some(full_line);
        } else if self.is_vertical() {
            for y in cmp::min(self.p1.y, self.p2.y)..=cmp::max(self.p1.y, self.p2.y) {
                full_line.push(Point { x: self.p1.x, y });
            }
            return Some(full_line);
        } else {
            let min_x = cmp::min(self.p1.x, self.p2.x);
            let max_x = cmp::max(self.p1.x, self.p2.x);

            let slope_x = max_x - min_x;
            let slope_y: i32;
            if max_x == self.p1.x {
                slope_y = self.p1.y - self.p2.y
            } else {
                slope_y = self.p2.y - self.p1.y
            }
            let b = self.p1.y - (self.p1.x * slope_y) / slope_x;
            for x in cmp::min(self.p1.x, self.p2.x)..=cmp::max(self.p1.x, self.p2.x) {
                full_line.push(Point {
                    x,
                    y: (x * slope_y) / slope_x + b,
                });
            }
            Some(full_line)
        }
    }
}

fn read_from_file(path: &str) -> Vec<LineSegment> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut line_list: Vec<LineSegment> = Vec::new();

    for line in reader.lines() {
        let res = line.unwrap();
        let first_split = res.split("->").collect::<Vec<&str>>();

        let first_pair = first_split[0]
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let first_point = Point {
            x: first_pair[0],
            y: first_pair[1],
        };

        let second_pair = first_split[1]
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let second_point = Point {
            x: second_pair[0],
            y: second_pair[1],
        };

        let line_segment = LineSegment {
            p1: first_point,
            p2: second_point,
        };
        line_list.push(line_segment);
    }
    line_list
}

fn print_map(map: &HashMap<Point, i32>, max_x: i32, max_y: i32) {
    println!("map");
    for y in 0..=max_y {
        for x in 0..=max_x {
            match map.get(&Point { x, y }) {
                Some(val) => print!("{}", val),
                None => print!("."),
            }
        }
        println!("");
    }
}

fn main() {
    let line_list = read_from_file("input/input.txt");

    let max_x = line_list
        .iter()
        .map(|p| cmp::max(p.p1.x, p.p2.x))
        .max()
        .unwrap();

    let max_y = line_list
        .iter()
        .map(|p| cmp::max(p.p1.y, p.p2.y))
        .max()
        .unwrap();

    let mut map = HashMap::new();
    for line in &line_list {
        if let Some(full_line) = line.extend() {
            for point in full_line {
                match map.get(&point) {
                    Some(val) => {
                        map.insert(point.clone(), val + 1);
                    }
                    None => {
                        map.insert(point.clone(), 1);
                    }
                }
            }
        }

        // print_map(&map, max_x, max_y);
    }

    let mut count = 0;
    for (_, val) in &map {
        if *val >= 2 {
            count += 1;
        }
    }

    println!("count: {:?}", count);
}
