use std::path::Path;
use std::fs::File;
use std::io::{Lines, BufReader, BufRead};

pub fn read_lines<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}