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

// Asummes not run on improper closes
fn find_proper_close(input: &str) -> Vec<char> {
    let mut open_stack = Vec::new();
    for c in input.chars() {
        if is_opening(c) {
            open_stack.push(c);
        }
        if is_closing(c) && is_proper_close(*open_stack.last().unwrap(), c) {
            open_stack.pop();
        }
    }

    let mut close_stack: Vec<char> = open_stack
        .iter()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("not a proper open"),
        })
        .collect();

    close_stack.reverse();
    println!("{:?}", close_stack);
    close_stack
}

fn main() {
    let input = read_from_file("input/input.txt");

    // Part 1
    let mut score: i64 = 0;
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

    let mut score_list = Vec::new();
    // Part 2
    for line in &input {
        score = 0;
        if find_improper_close(line).is_none() {
            find_proper_close(line).iter().for_each(|c| match c {
                ')' => score = score * 5 + 1,
                ']' => score = score * 5 + 2,
                '}' => score = score * 5 + 3,
                '>' => score = score * 5 + 4,
                _ => {}
            });

            score_list.push(score);
        }
    }
    score_list.sort_unstable();
    println!("score: {:?}", score_list);

    let mid = score_list.len() / 2;
    println!("score: {:?}", score_list[mid]);
}
