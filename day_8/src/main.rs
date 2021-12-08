use std::collections::HashMap;
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

fn has_all_of(input: &[char], inner: &[char]) -> bool {
    let count = inner
        .iter()
        .filter(|c| input.contains(c))
        .collect::<Vec<&char>>()
        .len();
    count >= inner.len()
}

fn union(vec_1: &[char], vec_2: &[char]) -> Vec<char> {
    let mut output = Vec::new();
    for item in vec_1 {
        output.push(*item);
    }

    for item in vec_2 {
        if !output.contains(item) {
            output.push(*item)
        }
    }

    output
}

fn get_key_map(temp: &Vec<Vec<char>>) -> HashMap<i32, Vec<char>> {
    let mut num_map: HashMap<i32, Vec<char>> = HashMap::new();

    // First decode fixed length numbers
    for item in temp {
        match item.len() {
            2 => num_map.insert(1, item.clone()),
            3 => num_map.insert(7, item.clone()),
            4 => num_map.insert(4, item.clone()),
            7 => num_map.insert(8, item.clone()),
            _ => None,
        };
    }

    let decoded_one = num_map.get(&1).unwrap().clone();

    // Deduce 3
    for item in temp {
        if item.len() == 5 && has_all_of(item, &decoded_one) {
            num_map.insert(3, item.clone());
        }
    }

    let decoded_three = num_map.get(&3).unwrap().clone();

    //Deduce 6, 0 and 9
    for item in temp {
        if item.len() == 6 {
            // 0 or 9
            if has_all_of(item, &decoded_one) {
                if union(item, &decoded_three).len() == 7 {
                    num_map.insert(0, item.clone());
                } else {
                    num_map.insert(9, item.clone());
                }
            } else {
                num_map.insert(6, item.clone());
            }
        }
    }

    let decoded_six = num_map.get(&6).unwrap().clone();
    // Deduce 2 and 5
    for item in temp {
        if item.len() == 5 && !has_all_of(item, &decoded_one) {
            if union(item, &decoded_six).len() == 7 {
                num_map.insert(2, item.clone());
            } else {
                num_map.insert(5, item.clone());
            }
        }
    }

    num_map
}

fn main() {
    let input = read_from_file("input/sample.txt");

    // Part 1
    let mut count = 0;
    for sequence in &input {
        for output in &sequence.output {
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

    let temp: Vec<Vec<char>> = input[0]
        .sequence
        .clone()
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let key_map = get_key_map(&temp);
    if key_map.len() != 10 {
        panic!("Not fully decoded");
    }
}
