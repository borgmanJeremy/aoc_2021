// Referencec this to learn iterators: https://github.com/timvisee/advent-of-code-2021/blob/master/day15a/src/main.rs

use pathfinding::prelude::dijkstra;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let input: Vec<Vec<_>> = reader
        .lines()
        .map(|line| line.unwrap().bytes().map(|c| (c - b'0') as i32).collect())
        .collect();

    input
}

fn main() {
    let input = read_from_file("input/input.txt");
    let start = (0, 0);
    let end = (input[0].len() as i32 - 1, input.len() as i32 - 1);
    println!("map: {:?}", input);
    let moves: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            moves
                .iter()
                .map(|(dx, dy)| {
                    input
                        .get((y + dy) as usize)
                        .and_then(|r| r.get((x + dx) as usize))
                        .map(|c| ((x + dx, y + dy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == end,
    )
    .unwrap()
    .1;

    println!("result: {:?}", result);
}
