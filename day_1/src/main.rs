use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<i32> = reader
        .lines()
        .map(|line| {
            if let Ok(text) = line {
                text.parse::<i32>().unwrap()
            } else {
                panic!("Failed to parse")
            }
        })
        .collect();

    data
}

fn main() {
    let input_data = read_from_file("input/input_1.txt");

    let part_1: i32 = input_data
        .windows(2)
        .map(|window| if window[0] < window[1] { 1 } else { 0 })
        .sum();

    println!("part 1: {}", part_1);

    let part_2: i32 = input_data
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|w| if w[0] < w[1] { 1 } else { 0 })
        .sum();

    println!("part 2: {:?}", part_2);
}
