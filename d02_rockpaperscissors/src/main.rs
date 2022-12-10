use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(PartialEq, Copy, Clone)]
enum GameResult {
    Lose = 0,
    Win = 6,
    Draw = 3
}

#[derive(PartialEq, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

fn decrypt_choice(encrypted_choice: &str) -> Option<Choice> {
    match encrypted_choice {
        "A" => Some(Choice::Rock),
        "B" => Some(Choice::Paper),
        "C" => Some(Choice::Scissors),
        _ => None
    }
}

fn decrypt_outcome(encrypted_outcome: &str) -> Option<GameResult> {
    match encrypted_outcome {
        "X" => Some(GameResult::Lose),
        "Y" => Some(GameResult::Draw),
        "Z" => Some(GameResult::Win),
        _ => None
    }
}

fn outcome_to_choice(opponent: Choice, outcome: GameResult) -> Choice {
    if game_result(Choice::Rock, opponent) == outcome {
        Choice::Rock
    } else if game_result(Choice::Paper, opponent) == outcome {
        Choice:: Paper
    } else {
        Choice::Scissors
    }
}

fn game_result(mine: Choice, opponent: Choice) -> GameResult {
    match (mine, opponent) {
        (mine, opponent) if mine == opponent => GameResult::Draw,
        (Choice::Rock, Choice::Scissors) => GameResult::Win,
        (Choice::Paper, Choice::Rock) => GameResult::Win,
        (Choice::Scissors, Choice::Paper) => GameResult::Win,
        _ => GameResult::Lose
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut points = 0;
    for line in reader.lines() {
        match line {
            Err(_) => {},
            Ok(line) => {
                let encrypted: Vec<&str> = line.split(" ").collect();
                let opponent = decrypt_choice(encrypted[0]).unwrap();
                let outcome = decrypt_outcome(encrypted[1]).unwrap();
                points += outcome as u32;
                let mine = outcome_to_choice(opponent, outcome);
                points += mine as u32;
            }
        }
    }

    println!("{points}");

    Ok(())
}
