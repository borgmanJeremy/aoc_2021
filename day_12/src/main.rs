use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug, PartialEq)]
enum Fold {
    XAxis(i32),
    YAxis(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct MapSize {
    width: i32,
    height: i32,
}

fn print_set(input: &HashSet<Point>, size: &MapSize) {
    for y in 0..size.height {
        for x in 0..size.width {
            if input.contains(&Point { x, y }) {
                print!("%");
            } else {
                print!(" ")
            }
        }
        println!();
    }
    println!();
}

fn fold_paper(input: &HashSet<Point>, fold: &Fold, size: &MapSize) -> (HashSet<Point>, MapSize) {
    let mut output = HashSet::new();
    let mut new_size = size.clone();

    match fold {
        Fold::YAxis(mag) => {
            for point in input {
                if point.x >= size.width / 2 {
                    output.insert(Point {
                        x: 2 * mag - point.x,
                        y: point.y,
                    });
                    new_size.width = size.width / 2;
                } else {
                    output.insert(Point {
                        x: point.x,
                        y: point.y,
                    });
                }
            }
        }
        Fold::XAxis(mag) => {
            for point in input {
                if point.y >= size.height / 2 {
                    output.insert(Point {
                        x: point.x,
                        y: 2 * mag - point.y,
                    });

                    new_size.height = size.height / 2;
                } else {
                    output.insert(Point {
                        x: point.x,
                        y: point.y,
                    });
                }
            }
        }
    }
    (output, new_size)
}

fn read_from_file(path: &str) -> (HashSet<Point>, Vec<Fold>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut point_set: HashSet<Point> = HashSet::new();
    let mut fold_list: Vec<Fold> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        if !text.is_empty() {
            if text.chars().nth(0).unwrap().is_digit(10) {
                let mut point_split = text.split(',');
                point_set.insert(Point {
                    x: point_split.next().unwrap().trim().parse::<i32>().unwrap(),
                    y: point_split.next().unwrap().trim().parse::<i32>().unwrap(),
                });
            } else {
                let mut fold_split = text.split_whitespace();
                if text.contains("x") {
                    let mut num_split = fold_split.nth(2).unwrap().split('=');
                    fold_list.push(Fold::YAxis(
                        num_split.nth(1).unwrap().trim().parse::<i32>().unwrap(),
                    ));
                } else if text.contains("y") {
                    let mut num_split = fold_split.nth(2).unwrap().split('=');
                    fold_list.push(Fold::XAxis(
                        num_split.nth(1).unwrap().trim().parse::<i32>().unwrap(),
                    ));
                }
            }
        }
    }
    (point_set, fold_list)
}

fn main() {
    let input = read_from_file("input/input.txt");

    let width = input
        .0
        .iter()
        .max_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
        .unwrap()
        .x;

    let height = input
        .0
        .iter()
        .max_by(|a, b| a.y.partial_cmp(&b.y).unwrap())
        .unwrap()
        .y;
    let starting_size = MapSize { width, height };
    // rather than using max val for each size, need to divide by 2 each time
    let mut folded = input.0.clone();
    let mut new_size = starting_size.clone();
    for fold in &input.1 {
        let res = fold_paper(&folded, fold, &new_size);
        folded = res.0;
        new_size = res.1;
        print_set(&folded, &new_size);
    }

    print_set(&folded, &new_size);
}
