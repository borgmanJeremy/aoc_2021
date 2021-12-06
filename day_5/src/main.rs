use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct LineSegment {
    p1: Point,
    p2: Point,
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
    let line_list = read_from_file("input/sample.txt");

    println!("{:?}", line_list);
}
