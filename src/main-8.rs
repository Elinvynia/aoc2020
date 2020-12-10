use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    let file = File::open("inputs/input-8.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    part_one(&lines);

    for x in 0..lines.len() {
        let line = lines.get(x).unwrap();
        if &line[0..3] == "nop" {
            let mut cloned = lines.clone();
            let change = cloned.get_mut(x).unwrap();
            let _ = std::mem::replace(change, format!("jmp {}", &line[4..]));
            if terminates(&cloned) {
                part_one(&cloned);
                break;
            }
        }
    }

    for x in 0..lines.len() {
        let line = lines.get(x).unwrap();
        if &line[0..3] == "jmp" {
            let mut cloned = lines.clone();
            let change = cloned.get_mut(x).unwrap();
            let _ = std::mem::replace(change, format!("nop {}", &line[4..]));
            if terminates(&cloned) {
                part_one(&cloned);
                break;
            }
        }
    }
}


fn part_one(lines: &Vec<String>) {
    let mut visited: Vec<usize> = vec![];
    let mut next: i32 = 0;
    let mut acc = 0;
    let last = lines.len();

    loop {
        if visited.contains(&(next as usize)) {
            break;
        }
        if next == last as i32 {
            break;
        }
        let line = lines.get(next as usize).unwrap();
        visited.push(next as usize);

        match &line[0..3] {
            "nop" => next += 1,
            "acc" => {
                let value = line[4..].parse::<i32>().unwrap();
                acc = acc + value;
                next += 1;
            },
            "jmp" => {
                let value = line[4..].parse::<i32>().unwrap();
                next = next + value;
            },
            _ => unreachable!(),
        };
    }

    println!("value of acc {:?}", acc);
}


fn terminates(lines: &Vec<String>) -> bool {
    let mut visited: Vec<usize> = vec![];
    let mut next: i32 = 0;
    let last = lines.len();

    loop {
        if visited.contains(&(next as usize)) {
            return false;
        }
        if next == last as i32 {
            return true
        }

        let line = lines.get(next as usize).unwrap();
        visited.push(next as usize);

        match &line[0..3] {
            "nop" => next += 1,
            "acc" => next += 1,
            "jmp" => {
                let value = line[4..].parse::<i32>().unwrap();
                next = next + value;
            },
            _ => unreachable!(),
        };
    }
}
