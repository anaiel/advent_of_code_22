use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;

const CRATE_LENGTH: usize = 4;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    
    let reader = BufReader::new(&file);
    let lines = reader.lines();
    let init = lines.take_while(|line| match line {
        Ok(line) => !line.is_empty(),
        Err(_) => false,
    }).map(|line| line.unwrap()).collect::<Vec<String>>();
    let nb_stacks = init.last().unwrap().chars().filter(|item| item.is_numeric()).collect::<Vec<char>>().len();
    let mut stacks_ref: Vec<Vec<char>> = vec![vec![]; nb_stacks];
    for line in init {
        line.char_indices()
            .for_each(|(i, crate_name)| if crate_name.is_alphabetic() { stacks_ref[i / CRATE_LENGTH].push(crate_name) });
    }
    stacks_ref.iter_mut().for_each(|stack| stack.reverse());

    let new_file = File::open("input")?;
    let new_reader = BufReader::new(new_file);
    let mut new_lines = new_reader.lines();
    let instr_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let mut stacks = stacks_ref.clone();
    while let Some(Ok(line)) = new_lines.next() {
        match instr_regex.captures(&line) {
            Some(matches) => {
                let crates_nb = matches[1].parse::<usize>().unwrap();
                let from = matches[2].parse::<usize>().unwrap() - 1;
                let to = matches[3].parse::<usize>().unwrap() - 1;
                for _ in 0..crates_nb {
                    let moving_crate = stacks[from].pop().unwrap();
                    stacks[to].push(moving_crate);
                }
            },
            _ => {}
        }
    }
    
    println!("First part: {:?}", stacks.iter().map(|stack| stack.last().unwrap()).collect::<Vec<&char>>());

    let new_file = File::open("input")?;
    let new_reader = BufReader::new(new_file);
    let mut new_lines = new_reader.lines();
    let instr_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let mut stacks = stacks_ref.clone();
    while let Some(Ok(line)) = new_lines.next() {
        match instr_regex.captures(&line) {
            Some(matches) => {
                let crates_nb = matches[1].parse::<usize>().unwrap();
                let from = matches[2].parse::<usize>().unwrap() - 1;
                let to = matches[3].parse::<usize>().unwrap() - 1;
                let to_move = &stacks[from][stacks[from].len() - crates_nb..];
                stacks[to] = [(&stacks[to]).to_vec(), to_move.to_vec()].concat();
                stacks[from] = (&stacks[from][..stacks[from].len() - crates_nb]).to_vec();
            },
            _ => {}
        }
    }
    
    println!("Second part: {:?}", stacks.iter().map(|stack| stack.last().unwrap()).collect::<Vec<&char>>());
    
    Ok(())
}
