
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_common_item(rucksacks: (&str, &str, &str)) -> Option<char> {
    for item0 in rucksacks.0.chars().collect::<Vec<char>>() {
        for item1 in rucksacks.1.chars().collect::<Vec<char>>() {
            if item0 == item1 {
                for item2 in rucksacks.2.chars().collect::<Vec<char>>() {
                    if item1 == item2 {
                        return Some(item0);
                    }
                }
            }
        }
    }
    None
}

fn priority(item: &char) -> Option<u32> {
    match item {
        item if item >= &'a' && item <= &'z' => Some(*item as u32 - 'a' as u32 + 1),
        item if item >= &'A' && item <= &'Z' => Some(*item as u32 - 'A' as u32 + 27),
        _ => None
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut priorities = 0;
    while let (Some(Ok(elf1)), Some(Ok(elf2)), Some(Ok(elf3))) = (lines.next(), lines.next(), lines.next()) {
        let common_item = find_common_item((&elf1, &elf2, &elf3)).unwrap();
        priorities += priority(&common_item).unwrap();
    }

    println!("{priorities}");

    Ok(())
}
