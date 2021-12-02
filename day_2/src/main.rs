use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn read_from_file(path: &str) -> Vec<(Direction, i32)> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<(Direction, i32)> = reader
        .lines()
        .map(|line| {
            if let Ok(text) = line {
                let dir_str: Vec<&str> = text.split(' ').collect();
                let dir = match dir_str[0] {
                    "forward" => Direction::Forward,
                    "down" => Direction::Down,
                    "up" => Direction::Up,
                    _ => panic!("Failed to Parse"),
                };
                (dir, dir_str[1].parse::<i32>().unwrap())
            } else {
                panic!("Failed to parse")
            }
        })
        .collect();

    data
}

fn main() {
    let input_data = read_from_file("input/input_1.txt");

    let mut hor_pos = 0;
    let mut vert_pos = 0;
    input_data.iter().for_each(|(dir, mag)| match dir {
        Direction::Forward => {
            hor_pos += mag;
        }
        Direction::Up => {
            vert_pos -= mag;
        }
        Direction::Down => {
            vert_pos += mag;
        }
    });

    println!(
        "Part 1 Horizontal: {}, Vertical: {}, Ans: {}",
        hor_pos,
        vert_pos,
        hor_pos * vert_pos
    );

    hor_pos = 0;
    vert_pos = 0;
    let mut aim = 0;
    input_data.iter().for_each(|(dir, mag)| match dir {
        Direction::Forward => {
            hor_pos += mag;
            vert_pos += aim * mag;
        }
        Direction::Up => {
            aim -= mag;
        }
        Direction::Down => {
            aim += mag;
        }
    });

    println!(
        "Part 2 Horizontal: {}, Vertical: {}, Ans: {}",
        hor_pos,
        vert_pos,
        hor_pos * vert_pos
    );
}
