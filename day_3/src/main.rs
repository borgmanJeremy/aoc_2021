use num::pow;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_from_file(path: &str) -> Vec<Vec<i32>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| match line {
            Ok(text) => text
                .chars()
                .map(|c| match c {
                    '0' => 0,
                    '1' => 1,
                    _ => panic!("failed to parse"),
                })
                .collect(),
            Err(_) => panic!("Failed to Parse"),
        })
        .collect();
    data
}

fn count_col(input: &Vec<Vec<i32>>, col: usize) -> i32 {
    let mut count = 0;
    for row in input {
        if row[col] == 1 {
            count += 1;
        }
    }
    count
}

fn main() {
    let input_data = read_from_file("input/input.txt");
    let nrow = input_data.len() as i32;
    let ncol = input_data[0].len() as i32;

    let sum = input_data
        .iter()
        .fold(vec![0; ncol as usize], |mut acc, x| {
            x.iter().enumerate().for_each(|(i, val)| acc[i] += val);
            acc
        });

    let gamma = sum.iter().enumerate().fold(0, |mut acc, (i, x)| {
        if *x > nrow / 2 {
            acc += pow(2, ncol as usize - i - 1);
        }
        acc
    });
    let epsilon = !(gamma as u32) & (0xFFFF >> (16 - ncol));

    println!("{:?}", sum);
    println!(
        "Part 1: gamma: {} epsilon: {} rate: {} ",
        gamma,
        epsilon,
        gamma * epsilon
    );

    let mut oxygen = input_data.clone();
    let mut active_col = 0;
    loop {
        oxygen = oxygen
            .iter()
            .cloned()
            .filter(|row| {
                let count = count_col(&oxygen, active_col);
                if count >= oxygen.len() as i32 - count {
                    row[active_col] == 1
                } else {
                    row[active_col] == 0
                }
            })
            .collect();
        active_col += 1;
        if oxygen.len() <= 1 {
            break;
        }
    }
    let mut co2 = input_data.clone();
    active_col = 0;
    loop {
        co2 = co2
            .iter()
            .cloned()
            .filter(|row| {
                let count = count_col(&co2, active_col);
                if count < co2.len() as i32 - count {
                    row[active_col] == 1
                } else {
                    row[active_col] == 0
                }
            })
            .collect();
        active_col += 1;
        if co2.len() <= 1 {
            break;
        }
    }

    let oxygen_sum = oxygen[0].iter().enumerate().fold(0, |mut acc, (i, x)| {
        if *x == 1 {
            acc += pow(2, ncol as usize - i - 1);
        }
        acc
    });

    let co2_sum = co2[0].iter().enumerate().fold(0, |mut acc, (i, x)| {
        if *x == 1 {
            acc += pow(2, ncol as usize - i - 1);
        }
        acc
    });
    println!(
        "oxy: {}, co2: {}, sum:{}",
        oxygen_sum,
        co2_sum,
        oxygen_sum * co2_sum
    );
}
