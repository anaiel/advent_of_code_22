use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{prelude::*, BufReader, Result};

const TOTAL: u32 = 70000000;
const REQUIRED: u32 = 30000000;

#[derive(Debug)]
struct Dir {
    dirs: Vec<String>,
    files: Vec<File>,
}
impl Dir {
    fn size(&self, dirs: &HashMap<String, Dir>) -> u32 {
        let files = match self.files.iter().map(|file| file.size).reduce(|a, b| a + b) {
            Some(size) => size,
            None => 0,
        };
        let subdirs = match self
            .dirs
            .iter()
            .map(|dir| dirs.get(dir).unwrap().size(dirs))
            .reduce(|a, b| a + b)
        {
            Some(sum) => sum,
            None => 0,
        };
        return files + subdirs;
    }
    fn new() -> Self {
        Dir {
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct File {
    size: u32,
}
impl File {
    fn new(size: u32) -> Self {
        Self { size }
    }
}

fn main() -> Result<()> {
    let file = fs::File::open("./input")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let cd_regex = Regex::new(r"\$ cd ([a-zA-Z0-9./]+)").unwrap();
    let ls_regex = Regex::new(r"\$ ls").unwrap();
    let file_regex = Regex::new("([0-9]+) [a-zA-Z0-9.]+").unwrap();
    let dir_regex = Regex::new("dir ([a-zA-Z0-9.]+)").unwrap();

    let mut dirs = HashMap::new();
    dirs.insert(String::from("/"), Dir::new());
    let mut current_folder = String::from("/");
    while let Some(Ok(line)) = lines.next() {
        if match cd_regex.captures(&line) {
            Some(matches) => {
                let folder_name = String::from(&matches[1]);
                match &folder_name {
                    string if string == "/" => {
                        current_folder = String::from("/");
                    }
                    string if string == ".." => {
                        let mut path = current_folder.split("/").collect::<Vec<&str>>().clone();
                        path.pop();
                        current_folder = if path.len() == 1 {
                            String::from("/")
                        } else {
                            String::from(path.join("/"))
                        };
                    }
                    name => {
                        let path = [String::from(&current_folder), name.to_string()]
                            .join(if &current_folder == "/" { "" } else { "/" });
                        let curr = dirs.get_mut(&current_folder).unwrap();
                        if !curr.dirs.iter().any(|dir| dir == &path) {
                            curr.dirs.push(String::from(&path));
                        }
                        if !dirs.contains_key(&path) {
                            dirs.insert(String::from(&path), Dir::new());
                        }
                        current_folder = path
                    }
                }
                true
            }
            _ => false,
        } {
            continue;
        }
        if ls_regex.is_match(&line) {
            continue;
        }
        if match file_regex.captures(&line) {
            Some(matches) => {
                let size = matches[1].parse::<u32>().unwrap();
                dirs.get_mut(&current_folder)
                    .unwrap()
                    .files
                    .push(File::new(size));
                true
            }
            _ => false,
        } {
            continue;
        }
        match dir_regex.captures(&line) {
            Some(matches) => {
                let folder = String::from(&matches[1]);
                if !dirs.contains_key(&folder) {
                    dirs.insert(folder, Dir::new());
                }
            }
            _ => {
                println!("{}", &line);
                panic!();
            }
        }
    }

    let total = dirs
        .values()
        .map(|dir| dir.size(&dirs))
        .filter(|size| *size <= 100000)
        .reduce(|a, b| a + b)
        .unwrap();

    println!("First part: {}", total);

    let unused = TOTAL - dirs.get("/").unwrap().size(&dirs);
    let to_free = REQUIRED - unused;
    let candidates = dirs
        .keys()
        .map(|dir| (dir, dirs.get(dir).unwrap().size(&dirs)))
        .filter(|(_, size)| *size >= to_free)
        .collect::<Vec<(&String, u32)>>();
    let mut min = candidates.first().unwrap();
    for dir in &candidates[1..] {
        if min.1 > dir.1 {
            min = dir;
        }
    }

    println!("Second part: {}", min.1);

    Ok(())
}
