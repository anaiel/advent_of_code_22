use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut fully_contained = 0;
    let mut overlap = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let sections = line
                    .split(",")
                    .map(|section| section
                        .split("-")
                        .map(|section_delimiter| section_delimiter.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                    )
                    .collect::<Vec<Vec<u32>>>();
                if fully_contains(&sections[0], &sections[1]) || fully_contains(&sections[1], &sections[0]) {
                    fully_contained += 1;
                }
                if overlaps(&sections[0], &sections[1]) {
                    overlap += 1;
                }
            },
            Err(_) => {}
        }
    }

    println!("part 1 answer: {fully_contained}");
    println!("part 2 answer: {overlap}");

    Ok(())
}

fn fully_contains(container: &Vec<u32>, containee: &Vec<u32>) -> bool {
    if container[0] <= containee[0] && container[1] >= containee[1] {
        true
    } else {
        false
    }
}

fn overlaps(section1: &Vec<u32>, section2: &Vec<u32>) -> bool {
    let first_section = if section1[0] <= section2[0] { section1 } else { section2 };
    let second_section = if section1[0] <= section2[0] { section2 } else { section1 };
    if first_section[0] == second_section[0] || first_section[1] >= second_section[0] {
        true
    } else {
        false
    }
}
