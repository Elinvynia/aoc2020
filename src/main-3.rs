use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-3.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let lines: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let mut trees_mul: u128 = 1;
    let mut index = 0;

    for x in &[1, 3, 5, 7] {
        let mut trees = 0;
        let mut lines = lines.clone();
        lines.remove(0);
        for line in lines {
            let length = line.len();
            index += x;
            if index >= length {
                index = index - length;
            }
            if line.as_bytes()[index] as char == '#' {
                trees += 1
            }
        }
        index = 0;
        trees_mul *= trees;
    }

    let mut trees = 0;

    let mut lines = lines.clone();
    lines.remove(0);
    lines.remove(0);
    for line in lines.into_iter().step_by(2) {
        let length = line.len();
        index += 1;
        if index >= length {
            index = index - length;
        }
        if line.as_bytes()[index] as char == '#' {
            trees += 1
        }
    }

    trees_mul *= trees;

    println!("{:?}", trees_mul);
}
