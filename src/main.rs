use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    for line in lines {
        let _line = line.unwrap();
        // parse logic here
    }
}
