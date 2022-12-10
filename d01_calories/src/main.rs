use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut top_3 = vec![0, 0, 0];
    let mut current_cal = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "" {
                    for max in &mut top_3 {
                        if current_cal > *max {
                            *max = current_cal;
                            break;
                        }
                    }
                    top_3.sort();
                    current_cal = 0;
                } else {
                    match line.parse::<u32>() {
                        Ok(value) => current_cal += value,
                        Err(_) => {}
                    }
                }
            },
            Err(_) => {}
        }
    }

    println!("{}", top_3[0] + top_3[1] + top_3[2]);

    Ok(())
}
