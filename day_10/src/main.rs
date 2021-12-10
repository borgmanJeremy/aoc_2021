use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<String> = reader.lines().map(|text| text.unwrap()).collect();

    data
}

fn is_opening(input: char) -> bool {
    matches!(input, '(' | '[' | '{' | '<')
}

fn is_closing(input: char) -> bool {
    matches!(input, ')' | ']' | '}' | '>')
}

fn is_proper_close(open: char, close: char) -> bool {
    match open {
        '(' => close == ')',
        '[' => close == ']',
        '{' => close == '}',
        '<' => close == '>',
        _ => panic!("not a proper open"),
    }
}

fn find_improper_close(input: &str) -> Option<char> {
    let mut open_stack = Vec::new();
    for c in input.chars() {
        if is_opening(c) {
            open_stack.push(c);
        }
        if is_closing(c) {
            if is_proper_close(*open_stack.last().unwrap(), c) {
                open_stack.pop();
            } else {
                return Some(c);
            }
        }
    }
    None
}

fn main() {
    let input = read_from_file("input/input.txt");

    let mut score = 0;

    for line in &input {
        if let Some(c) = find_improper_close(line) {
            match c {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => {}
            }
        };
    }
    println!("score: {}", score);
}
