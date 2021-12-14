use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};
#[derive(Debug, Clone, PartialEq)]
struct Formula {
    input: Vec<char>,
    output: char,
}

fn lookup_recipe(input: &Vec<char>, formulas: &Vec<Formula>) -> Option<char> {
    for recipe in formulas {
        if *input == recipe.input {
            return Some(recipe.output);
        }
    }
    None
}

fn read_from_file(path: &str) -> (Vec<Formula>, Vec<char>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut line_it = reader.lines();

    let start_chain: Vec<char> = line_it.next().unwrap().unwrap().chars().collect();
    // Skip blank line
    line_it.next();

    let mut formula_list: Vec<Formula> = Vec::new();

    for line in line_it {
        let temp = line.unwrap();
        let split: Vec<&str> = temp.split(" -> ").collect();

        let input: Vec<char> = split[0].chars().collect();
        let output: char = split[1].chars().collect::<Vec<char>>()[0];

        formula_list.push(Formula { input, output });
    }

    (formula_list, start_chain)
}
fn main() {
    let input = read_from_file("input/sample.txt");
    let formula_list = input.0;
    let starting_chain = input.1;

    println!("Start Chain: {:?}", starting_chain);
    println!("Formulas: {:?}", formula_list);

    let mut atom_list = Vec::new();
    for idx in 0..starting_chain.len() - 1 {
        let pair = vec![starting_chain[idx], starting_chain[idx + 1]];
        let atom = lookup_recipe(&pair, &formula_list).unwrap();
        atom_list.push(atom);
    }

    let mut new_chain = Vec::new();
    for idx in 0..starting_chain.len() {
        new_chain.push(starting_chain[idx]);
        if idx < atom_list.len() {
            new_chain.push(atom_list[idx]);
        }
    }
    println!("{:?}", new_chain);
}
