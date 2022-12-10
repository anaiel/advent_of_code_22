use std::fs::File;
use std::io::{BufReader, prelude::*, Result};

const PACKET_MARKER_LENGTH: usize = 4;
const MESSAGE_MARKER_LENGTH: usize = 14;

fn main() -> Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let signal = reader.lines().next().unwrap().unwrap();
    

    println!("First part: {:?}", index_of_nth_consecutive_different_letter(&signal, PACKET_MARKER_LENGTH));
    println!("First part: {:?}", index_of_nth_consecutive_different_letter(&signal, MESSAGE_MARKER_LENGTH));

    Ok(())
}

fn has_duplicate_letter(slice: &str) -> bool {
    for (i, letter) in slice.char_indices() {
        if slice[i+1..].chars().any(|next_letter| next_letter == letter) {
            return true;
        }
    }
    false
}

fn index_of_nth_consecutive_different_letter(signal: &str, length: usize) -> usize {
    let mut nb_char: Option<usize> = None;
    for i in 0..signal.len() {
        if i < length { continue; }
        if has_duplicate_letter(&signal[i-length..i]) {
            continue;
        } else {
            nb_char = Some(i);
            break;
        }
    }
    match nb_char {
        Some(nb_char) => nb_char,
        None => panic!()
    }
}
