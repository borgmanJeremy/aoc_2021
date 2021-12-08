use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct NumberSequence {
    sequence: Vec<String>,
    output: Vec<String>,
}

fn read_from_file(path: &str) -> Vec<NumberSequence> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut input: Vec<NumberSequence> = Vec::new();
    for line in reader.lines() {
        let res = line.unwrap();
        let first_split = res.split('|').collect::<Vec<&str>>();
        let sequence = first_split[0]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let output = first_split[1]
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        input.push(NumberSequence { sequence, output });
    }

    input
}

fn main() {
    let input = read_from_file("input/input.txt");
    println!("{:?}", input);

    let mut count = 0;
    for sequence in input {
        for output in sequence.output {
            match output.len() {
                2 => count += 1,
                3 => count += 1,
                4 => count += 1,
                7 => count += 1,
                _ => {}
            }
        }
    }

    println!("Count: {}", count);
}
