use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};
fn read_from_file(path: &str) -> Vec<i32> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut input = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    input.sort_unstable();

    input
}

fn main() {
    let input = read_from_file("input/input.txt");
    let min_pos = *input.iter().min().unwrap();
    let max_pos = *input.iter().max().unwrap();

    let mut fuel_cost = std::i32::MAX;

    for pos in min_pos..=max_pos {
        let mut cur_fuel_cost = 0;
        for crab in &input {
            cur_fuel_cost += (crab - pos).abs();
        }
        if cur_fuel_cost < fuel_cost {
            fuel_cost = cur_fuel_cost;
        }
    }

    println!("{:?}", input);

    println!("Fuel Cost: {:?}", fuel_cost);
}
