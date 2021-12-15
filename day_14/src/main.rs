use std::collections::HashMap;
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
    let input = read_from_file("input/input.txt");
    let formula_list = input.0;
    let starting_chain = input.1;

    // println!("Start Chain: {:?}", starting_chain);
    // println!("Formulas: {:?}", formula_list);

    let mut new_chain = starting_chain.clone();
    for _step in 0..40 {
        println!("{}", _step);
        let mut atom_list = Vec::new();

        for idx in 0..new_chain.len() - 1 {
            let pair = vec![new_chain[idx], new_chain[idx + 1]];
            let atom = lookup_recipe(&pair, &formula_list).unwrap();
            atom_list.push(atom);
        }

        let mut tmp_chain = Vec::new();
        for idx in 0..new_chain.len() {
            tmp_chain.push(new_chain[idx]);
            if idx < atom_list.len() {
                tmp_chain.push(atom_list[idx]);
            }
        }

        new_chain = tmp_chain;
    }

    let mut atom_count = HashMap::new();
    for atom in &new_chain {
        if atom_count.contains_key(atom) {
            atom_count.insert(atom, atom_count.get(atom).unwrap() + 1);
        } else {
            atom_count.insert(atom, 1);
        }
    }
    let mut min_val = i32::MAX;
    let mut max_val = 0;
    let mut min_key = 'x';
    let mut max_key = 'x';

    for (k, v) in &atom_count {
        if *v < min_val {
            min_val = *v;
            min_key = **k;
        }

        if *v > max_val {
            max_val = *v;
            max_key = **k;
        }
    }
    println!("{:?}", atom_count);

    println!(
        "max(k,v): ({},{}) min(k,v): ({},{})",
        max_key, max_val, min_key, min_val
    );
}
