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

fn expand_map(input: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    const ITER: i32 = 5;

    let mut extended_column = Vec::new();
    for row in input {
        let mut new_row = Vec::new();
        for iter in 0..ITER {
            for val in row.iter() {
                new_row.push((*val + iter - 1) % 9 + 1);
            }
        }
        extended_column.push(new_row.clone());
    }

    let mut extended_row = Vec::new();

    for iter in 0..ITER {
        for row in &extended_column {
            extended_row.push(row.clone().iter().map(|v| (v + iter - 1) % 9 + 1).collect());
        }
    }

    extended_row
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
    let expanded_map = expand_map(&input);

    let end_2 = (
        expanded_map[0].len() as i32 - 1,
        expanded_map.len() as i32 - 1,
    );
    let part_2 = dijkstra(
        &(0, 0),
        |&(x, y)| {
            moves
                .iter()
                .map(|(dx, dy)| {
                    expanded_map
                        .get((y + dy) as usize)
                        .and_then(|r| r.get((x + dx) as usize))
                        .map(|c| ((x + dx, y + dy), *c as u32))
                })
                .flatten()
                .collect::<Vec<_>>()
        },
        |&p| p == end_2,
    )
    .unwrap()
    .1;

    println!("part 2: {:?}", part_2);
}
