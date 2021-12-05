use std::fs::File;
use std::io::{prelude::*, BufReader};

const SIZE: usize = 5;

#[derive(Clone, Debug, PartialEq)]
struct BingoSquare {
    tile_number: i32,
    has_been_called: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct BingoBoard {
    board: Vec<BingoSquare>,
}

fn read_from_file(path: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut to_be_called = Vec::new();

    let mut temp_board = Vec::new();
    let mut board_list: Vec<BingoBoard> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        if i == 0 {
            line.as_ref()
                .unwrap()
                .split(',')
                .for_each(|x| to_be_called.push(x.parse::<i32>().unwrap()));
        }

        if !line.as_ref().unwrap().is_empty() && i != 0 {
            line.unwrap()
                .split_whitespace()
                .for_each(|c| temp_board.push(c.parse::<i32>().unwrap()));
        } else {
            if !temp_board.is_empty() {
                board_list.push(BingoBoard::new(&temp_board.clone()));
                temp_board.clear();
            }
        }
    }

    board_list.push(BingoBoard::new(&temp_board.clone()));
    (to_be_called, board_list)
}

impl BingoBoard {
    fn new(input: &Vec<i32>) -> Self {
        let mut init = Vec::new();
        input.iter().for_each(|x| {
            init.push(BingoSquare {
                tile_number: *x,
                has_been_called: false,
            })
        });
        Self { board: init }
    }

    fn call(&mut self, input: i32) {
        self.board.iter_mut().for_each(|x| {
            if x.tile_number == input {
                x.has_been_called = true;
            }
        });
    }

    fn check_row(&self) -> Option<Vec<i32>> {
        let mut count = 0;
        let mut winning_line = Vec::new();
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.board[col + row * SIZE].has_been_called == true {
                    count += 1;
                    winning_line.push(self.board[col + row * SIZE].tile_number);
                }
            }
            if count == SIZE {
                return Some(winning_line);
            }
            count = 0;
            winning_line.clear();
        }
        None
    }

    fn check_col(&self) -> Option<Vec<i32>> {
        let mut count = 0;
        let mut winning_line = Vec::new();
        for col in 0..SIZE {
            for row in 0..SIZE {
                if self.board[col + row * SIZE].has_been_called == true {
                    winning_line.push(self.board[col + row * SIZE].tile_number);
                    count += 1;
                }
            }
            if count == SIZE {
                return Some(winning_line);
            }
            count = 0;
            winning_line.clear();
        }
        None
    }

    fn is_winner(&self) -> Option<Vec<i32>> {
        match self.check_col() {
            Some(winning_line) => Some(winning_line),
            None => match self.check_row() {
                Some(winning_line) => Some(winning_line),
                None => {
                    return None;
                }
            },
        }
    }
}

fn main() {
    let (call_list, mut board_list) = read_from_file("input/input.txt");
    let mut winner_found = false;

    // Part 1
    for call in &call_list {
        for board in &mut board_list {
            board.call(*call);
            if let Some(winner) = board.is_winner() {
                let sum = board.board.iter().fold(0, |acc, i| {
                    if !i.has_been_called {
                        acc + i.tile_number
                    } else {
                        acc
                    }
                });

                println!("board: {:?}", board);
                println!("call: {:?}", call);
                println!("winning line: {:?}", winner);
                println!("sum uncalled: {:?}", sum);
                println!("Part 1: {:?}", sum * call);
                winner_found = true;
                break;
            }
        }
        if winner_found {
            break;
        }
    }

    // Part 2
    for call in &call_list {
        for board in &mut board_list {
            board.call(*call);
        }
        if (board_list.len() > 1) {
            board_list = board_list
                .into_iter()
                .filter(|board| {
                    if let Some(_) = board.is_winner() {
                        false
                    } else {
                        true
                    }
                })
                .collect();
        }

        if board_list.len() == 1 {
            if let Some(winner) = board_list[0].is_winner() {
                let sum = board_list[0].board.iter().fold(0, |acc, i| {
                    if !i.has_been_called {
                        acc + i.tile_number
                    } else {
                        acc
                    }
                });
                println!("is winner: {:?}", board_list[0].is_winner());
                println!("call: {}", call);
                println!("sum: {}", sum);
                println!("Part 2: {:?}", sum * call);

                break;
            }
        }
    }
}
