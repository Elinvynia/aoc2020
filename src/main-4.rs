use std::fs;

fn main() {
    let file = fs::read_to_string("inputs/input-4.txt").unwrap();
    let mut passports: Vec<String> = file.split("\n\n").map(|a| a.replace("\n", " ")).collect();
    let last = passports.last_mut().unwrap();
    last.truncate(last.len() - 1);

    let mut total = 0;

    for passport in passports.iter() {
        let indices: Vec<&str> = passport.split(" ").map(|x| x.get(0..3).unwrap()).collect();

        if indices.len() < 7 {
            continue
        }

        if (indices.len() == 7 && !indices.contains(&"cid")) || indices.len() == 8 {
            let data: Vec<Vec<String>> = passport.split(" ").map(|a| a.split(":").map(|a| a.to_string()).collect()).collect();
            let mut ok = 0;
            for field in data {
                match &field[0][..] {
                    "byr" => {
                        if field[1].len() == 4 {
                            match field[1].parse::<u32>() {
                                Ok(v) => {
                                    if 1920 <= v && v <= 2002 {
                                        ok += 1
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                    },
                    "iyr" => {
                        if field[1].len() == 4 {
                            match field[1].parse::<u32>() {
                                Ok(v) => {
                                    if 2010 <= v && v <= 2020 {
                                        ok += 1
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                    },
                    "eyr" => {
                        if field[1].len() == 4 {
                            match field[1].parse::<u32>() {
                                Ok(v) => {
                                    if 2020 <= v && v <= 2030 {
                                        ok += 1
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                    },
                    "hgt" => {
                        let mut chars: Vec<char> = field[1].chars().collect();
                        if chars.len() < 4 {
                            continue;
                        };
                        chars.reverse();
                        let mut chars2 = chars.clone();
                        if chars.get(1).unwrap() == &'c' && chars.get(0).unwrap() == &'m' {
                            chars.reverse();
                            chars.truncate(3);
                            match chars.into_iter().collect::<String>().parse::<u8>() {
                                Ok(v) => {
                                    if 150 <= v && v <= 193 {
                                        ok += 1
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                        if chars2.get(1).unwrap() == &'i' && chars2.get(0).unwrap() == &'n' {
                            chars2.reverse();
                            chars2.truncate(2);
                            match chars2.into_iter().collect::<String>().parse::<u8>() {
                                Ok(v) => {
                                    if 59 <= v && v <= 76 {
                                        ok += 1
                                    }
                                },
                                Err(_) => continue,
                            }
                        }
                    },
                    "hcl" => {
                        if field[1].starts_with("#") {
                            let mut all_valid = true;
                            for (i, x) in field[1].chars().enumerate() {
                                if i == 0 {
                                    continue
                                }
                                if !["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"].contains(&&x.to_string()[..]) {
                                    all_valid = false;
                                    break;
                                }
                            }
                            if all_valid {
                                ok += 1
                            }
                        }
                    },
                    "ecl" => {
                        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&field[1][..]) {
                            ok += 1;
                        }
                    },
                    "pid" => {
                        if field[1].len() == 9 && field[1].parse::<u64>().is_ok() {
                            ok += 1;
                        }
                    },
                    "cid" => continue,
                    _ => continue,
                }
            }
            if ok == 7 {
                total += 1
            }
        }
    }

    println!("{:?}", total);
}
