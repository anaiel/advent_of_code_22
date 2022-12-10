use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

const INTERESTING_SIGNALS: [i32; 6] = [20, 60, 100, 140, 180, 220];

fn main() -> Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut x = 1;
    let mut cycle = 1;
    let mut sum = 0;
    let addx = Regex::new("addx (-?[0-9]+)").unwrap();
    let mut crt: Vec<Vec<char>> = vec![Vec::new(); 6];
    while let Some(Ok(line)) = lines.next() {
        let mut duration = 1;
        let mut x_diff = 0;
        match addx.captures(&line) {
            Some(matches) => {
                duration = 2;
                x_diff = matches[1].parse::<i32>().unwrap();
            }
            None => {}
        }
        if let Some(signal) = interesting_signal(cycle, duration) {
            sum += signal * x;
        }
        let mut i = 0;
        while i < duration && (cycle + i - 1) / 40 < 6 {
            println!("cycle {}: x {x}", cycle + i);
            crt[((cycle + i - 1) / 40) as usize].push(
                if (cycle % 40) + i >= x && (cycle % 40) + i <= x + 2 {
                    '#'
                } else {
                    '.'
                },
            );
            i += 1;
        }
        cycle += duration;
        x += x_diff;

        if cycle > 240 {
            break;
        }
    }

    println!("First part: {}", sum);
    println!("Second part");
    for line in crt {
        println!("{}", line.iter().collect::<String>());
    }

    Ok(())
}

fn interesting_signal(cycle: i32, duration: i32) -> Option<i32> {
    for signal in INTERESTING_SIGNALS {
        if cycle <= signal && cycle + duration > signal {
            return Some(signal);
        }
    }
    None
}
