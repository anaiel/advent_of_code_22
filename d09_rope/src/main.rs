use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Knot {
    pos: (i32, i32),
}
impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Knot {
            pos: (x, y)
        }
    }

    fn move_towards(&self, dir: &str) -> Self {
        match dir {
            "R" => Knot::new(self.pos.0 + 1, self.pos.1),
            "L" => Knot::new(self.pos.0 - 1, self.pos.1),
            "U" => Knot::new(self.pos.0, self.pos.1 + 1),
            "D" => Knot::new(self.pos.0, self.pos.1 - 1),
            _ => panic!(),
        }
    }

    fn adjacent(&self, other: &Self) -> bool {
        (self.pos.0 - other.pos.0).abs() <= 1 && (self.pos.1 - other.pos.1).abs() <= 1
    }

    fn catch_up(&self, other: &Self) -> Self {
        if self.pos.0 == other.pos.0 {
            Knot::new(
                self.pos.0,
                if self.pos.1 > other.pos.1 {
                    self.pos.1 - 1
                } else {
                    self.pos.1 + 1
                },
            )
        } else if self.pos.1 == other.pos.1 {
            Knot::new(
                if self.pos.0 > other.pos.0 {
                    self.pos.0 - 1
                } else {
                    self.pos.0 + 1
                },
                self.pos.1,
            )
        } else {
            Knot::new(
                if self.pos.0 > other.pos.0 {
                    self.pos.0 - 1
                } else {
                    self.pos.0 + 1
                },
                if self.pos.1 > other.pos.1 {
                    self.pos.1 - 1
                } else {
                    self.pos.1 + 1
                },
            )
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut rope = vec![vec![Knot::new(0, 0);10]];
    let instr = Regex::new("([RDLU]) ([0-9]+)").unwrap();
    while let Some(Ok(line)) = lines.next() {
        match instr.captures(&line) {
            None => panic!(),
            Some(matches) => {
                let dir = &matches[1];
                let mut nb_steps = matches[2].parse::<u32>().unwrap();
                while nb_steps > 0 {
                    let curr_rope = rope.last().unwrap();
                    let mut new_rope: Vec<Knot> = vec![curr_rope.first().unwrap().move_towards(dir)];
                    let mut i = 1;
                    while i < curr_rope.len() {
                        let new_knot = if !curr_rope[i].adjacent(&new_rope[i - 1]) {
                            curr_rope[i].catch_up(&new_rope[i - 1])
                        } else {
                            curr_rope[i].clone()
                        };
                        new_rope.push(new_knot);
                        i += 1;
                    }
                    rope.push(new_rope);
                    nb_steps -= 1;
                }
            }
        }
    }

    let first_knot = rope.iter().map(|rope_pos| &rope_pos[1]).collect::<Vec<&Knot>>();
    let mut first_unique: Vec<&Knot> = Vec::new();
    for pos in first_knot {
        if !first_unique.iter().any(|p| *p == pos) {
            first_unique.push(pos);
        }
    }

    let ninth_knot = rope.iter().map(|rope_pos| &rope_pos[9]).collect::<Vec<&Knot>>();
    let mut ninth_unique: Vec<&Knot> = Vec::new();
    for pos in ninth_knot {
        if !ninth_unique.iter().any(|p| *p == pos) {
            ninth_unique.push(pos);
        }
    }

    println!("First part: {}", first_unique.len());
    println!("First part: {}", ninth_unique.len());

    Ok(())
}
