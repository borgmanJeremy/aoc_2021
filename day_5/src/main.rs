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

    fn is_straight(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    fn extend(&self) -> Option<Vec<Point>> {
        let mut full_line = Vec::new();
        if self.is_horizontal() {
            for x in cmp::min(self.p1.x, self.p2.x)..=cmp::max(self.p1.x, self.p2.x) {
                full_line.push(Point { x, y: self.p1.y });
            }
            return Some(full_line);
        } else if self.is_vertical() {
            println!("{} {} ", self.p1.y, self.p2.y);
            for y in cmp::min(self.p1.y, self.p2.y)..=cmp::max(self.p1.y, self.p2.y) {
                println!("{}", y);
                full_line.push(Point { x: self.p1.x, y });
            }
            return Some(full_line);
        }
        None
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
    }

    let mut count = 0;
    for (_, val) in &map {
        if *val >= 2 {
            count += 1;
        }
    }

    println!("{:?}", map);
    println!("count: {:?}", count);
}
