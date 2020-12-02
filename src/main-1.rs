use std::{fs::File, io::{BufRead, BufReader}, vec};

fn main() {
    let file = File::open("inputs/input-1.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut numbers: Vec<u32> = vec![];

    for line in lines {
        let line = line.unwrap();
        let number: u32 = line.parse().unwrap();
        numbers.push(number);
    }

    for x in numbers.iter() {
        for y in numbers.iter() {
            for z in numbers.iter() {
                if x + y + z == 2020 {
                    panic!("{}", x * y * z);
                }
            }
        }
    }
}
