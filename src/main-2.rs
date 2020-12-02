use std::{fs::File, io::{BufRead, BufReader}, vec};

struct Password {
    letter: char,
    min_amount: u8,
    max_amount: u8,
    password: String,
}

fn main() {
    let file = File::open("inputs/input-2.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut passwords: Vec<Password> = vec![];

    for line in lines {
        let line = line.unwrap();
        let left: Vec<&str> = line.split(": ").collect();
        let password = left[1].to_string();
        let right: Vec<&str> = left[0].split(" ").collect();
        let letter = right[1].chars().collect::<Vec<char>>()[0];
        let limits: Vec<&str> = right[0].split("-").collect();
        let min_amount: u8 = limits[0].parse().unwrap();
        let max_amount: u8 = limits[1].parse().unwrap();

        let password = Password {
            letter,
            min_amount,
            max_amount,
            password,
        };

        passwords.push(password);
    }

    let mut total = 0;

    for password in passwords.iter() {
        let mut amount = 0;
        for x in password.password.chars() {
            if x == password.letter {
                amount += 1
            }
        }
        if (password.min_amount <= amount) && (amount <= password.max_amount) {
            total += 1
        }
    }

    let mut total2 = 0;

    for password in passwords.iter() {
        let mut amount = 0;
        for (i, x) in password.password.chars().enumerate() {
            if x == password.letter {
                if i + 1 == password.min_amount as usize {
                    amount += 1
                };
                if i + 1 == password.max_amount as usize {
                    amount += 1
                };
            }
        }
        if amount == 1 {
            total2 += 1
        }
    }

    println!("part 1 {:?}", total);
    println!("part 2 {:?}", total2);
}
