use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
struct FishPopulation {
    population: Vec<i32>,
}

impl FishPopulation {
    fn new() -> Self {
        let mut population = Vec::new();
        for _day in 0..=8 {
            population.push(0);
        }
        Self { population }
    }

    fn step(&mut self) {
        let new_fish_count = self.population[0];

        for day in 0..8 {
            self.population[day] = self.population[day + 1];
        }
        self.population[6] += new_fish_count;
        self.population[8] = new_fish_count;
    }
}

fn read_from_file(path: &str) -> FishPopulation {
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

    let mut population = FishPopulation::new();
    for fish in input {
        population.population[fish as usize] += 1;
    }

    population
}

fn main() {
    let mut population = read_from_file("input/input.txt");

    println!("{:?}", population);
    for day in 0..80 {
        population.step();
        println!("Day: {}, {:?}", day, population);
    }

    println!(
        "sum: {}",
        population.population.iter().fold(0, |sum, x| (sum + x))
    )
}
