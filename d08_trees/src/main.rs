use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut trees: Vec<Vec<u32>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let row: Vec<u32> = line
            .chars()
            .map(|height| height.to_digit(10).unwrap())
            .collect();
        trees.push(row);
    }

    let from_left = visible_from_left(&trees);

    let mut _trees = rotate(&trees);
    let _from_top = visible_from_left(&_trees);
    let from_top = _from_top
        .iter()
        .map(|(x, y)| (*y, *x))
        .collect::<Vec<(usize, usize)>>();

    trees.iter_mut().for_each(|row| row.reverse());
    let _from_right = visible_from_left(&trees);
    let from_right = _from_right
        .iter()
        .map(|(x, y)| (trees[0].len() - 1 - x, *y))
        .collect::<Vec<(usize, usize)>>();

    _trees.iter_mut().for_each(|row| row.reverse());
    let _from_bottom = visible_from_left(&_trees);
    let from_bottom = _from_bottom
        .iter()
        .map(|(x, y)| (_trees[0].len() - 1 - x, *y))
        .map(|(x, y)| (y, x))
        .collect::<Vec<(usize, usize)>>();

    let all = [from_top, from_right, from_left, from_bottom].concat();
    let mut unique = Vec::new();
    for tree in all {
        if !unique.iter().any(|(x, y)| tree.0 == *x && tree.1 == *y) {
            unique.push(tree);
        }
    }

    let mut max = 0;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[y].len() - 1 {
            let mut score = 1;

            // to the right
            let mut i = 1;
            while x + i < trees[y].len() && trees[y][x + i] < trees[y][x] {
                i += 1;
            }
            if x + i == trees[y].len() {
                i -= 1;
            }
            score *= i;

            // to the left
            let mut i = 1;
            while x >= i && trees[y][x - i] < trees[y][x] {
                i += 1;
            }
            if i == x + 1 {
                i -= 1;
            }
            score *= i;

            // to the top
            let mut i = 1;
            while y + i < trees.len() && trees[y + i][x] < trees[y][x] {
                i += 1;
            }
            if y + i == trees.len() {
                i -= 1;
            }
            score *= i;

            // to the bottom
            let mut i = 1;
            while y >= i && trees[y - i][x] < trees[y][x] {
                i += 1;
            }
            if i == y + 1 {
                i -= 1;
            }
            score *= i;

            if score > max {
                max = score;
            }
        }
    }

    println!("First part: {}", unique.len());
    println!("Second part: {}", max);

    Ok(())
}

fn visible_from_left(trees: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut visible: Vec<(usize, usize)> = Vec::new();
    for y in 0..trees.len() {
        let mut max_height: Option<u32> = None;
        for x in 0..trees[y].len() {
            match max_height {
                Some(val) => {
                    if val < trees[y][x] {
                        visible.push((x, y));
                        max_height = Some(trees[y][x])
                    }
                }
                None => {
                    visible.push((x, y));
                    max_height = Some(trees[y][x])
                }
            }
        }
    }
    visible
}

fn rotate(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut rotated: Vec<Vec<u32>> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            match rotated.get_mut(x) {
                Some(row) => {
                    row.push(grid[y][x]);
                }
                None => rotated.push(vec![grid[y][x]]),
            }
        }
    }
    rotated
}
